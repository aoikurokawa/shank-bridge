use std::{fs::File, io::Read, sync::Arc};

use axum::{
    extract::{
        ws::{Message, WebSocket},
        State, WebSocketUpgrade,
    },
    response::IntoResponse,
    routing::get,
    Router,
};
use futures::{sink::SinkExt, stream::StreamExt};
use rig::{
    agent::Agent,
    completion::Prompt,
    providers::anthropic::{self, completion::CompletionModel, CLAUDE_3_HAIKU},
};
use tokio::sync::broadcast;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();

    // Set up application state for use with with_state().
    let (tx, _rx) = broadcast::channel(100);

    let path = std::env::var("PROMPT_FILE_PATH")?;

    let client = anthropic::ClientBuilder::new(
        &std::env::var("ANTHROPIC_API_KEY").expect("ANTHROPIC_API_KEY not set"),
    )
    .build();

    let mut prompt_file = File::open(&path).unwrap();
    let mut contents = String::new();
    prompt_file.read_to_string(&mut contents).unwrap();

    // Create agent with a single context prompt
    let agent = client
        .agent(CLAUDE_3_HAIKU)
        .preamble(&contents)
        .temperature(0.5)
        .max_tokens(4096)
        .build();

    let app_state = Arc::new(AppState { agent, tx });

    let app = Router::new()
        .route("/", get("Hello, World!"))
        .route("/websocket", get(websocket_handler))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

// Our shared state
struct AppState {
    agent: Agent<CompletionModel>,

    // Channel used to send messages to all connected clients.
    tx: broadcast::Sender<String>,
}

async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| websocket(socket, state))
}

// This function deals with a single websocket connection, i.e., a single
// connected client / user, for which we will spawn two independent tasks (for
// receiving / sending chat messages).
async fn websocket(stream: WebSocket, state: Arc<AppState>) {
    // By splitting, we can send and receive at the same time.
    let (mut sender, mut receiver) = stream.split();

    // We subscribe *before* sending the "joined" message, so that we will also
    // display it to our client.
    let mut rx = state.tx.subscribe();

    // Spawn the first task that will receive broadcast messages and send text
    // messages over the websocket to our client.
    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            // In any websocket error, break loop.
            if sender.send(Message::Text(msg)).await.is_err() {
                break;
            }
        }
    });

    // Clone things we want to pass (move) to the receiving task.
    let tx = state.tx.clone();

    // Spawn a task that takes messages from the websocket, prepends the user
    // name, and sends them to all broadcast subscribers.
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(Message::Text(text))) = receiver.next().await {
            // Prompt the agent and print the response
            let response = state.agent.prompt(&text).await.unwrap();

            // Add username before message.
            let _ = tx.send(response);
            let _ = tx.send(String::from("[DONE]"));
        }
    });

    // If any one of the tasks run to completion, we abort the other.
    tokio::select! {
        _ = &mut send_task => recv_task.abort(),
        _ = &mut recv_task => send_task.abort(),
    };
}
