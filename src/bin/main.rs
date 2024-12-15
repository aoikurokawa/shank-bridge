use std::{fs::File, io::Read, sync::Arc, time::Duration};

use axum::{
    body::Bytes,
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing, Json, Router,
};
use futures::{SinkExt, StreamExt};
use ncn_portal::MessageRequest;
use rig::{
    agent::Agent,
    completion::Prompt,
    providers::anthropic::{self, completion::CompletionModel, CLAUDE_3_5_SONNET},
};
use tower_http::trace::TraceLayer;
use tracing::Span;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

// Our shared state
struct AppState {
    agent: Agent<CompletionModel>,
}

// Make our own error that wraps `anyhow::Error`.
struct AppError(anyhow::Error);

// Tell axum how to convert `AppError` into a response.
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        )
            .into_response()
    }
}

// This enables using `?` on functions that return `Result<_, anyhow::Error>` to turn them into
// `Result<_, AppError>`. That way you don't need to do that manually.
impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!("{}=debug,tower_http=debug", env!("CARGO_CRATE_NAME")).into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Create client
    let client = anthropic::ClientBuilder::new(
        &std::env::var("ANTHROPIC_API_KEY").expect("ANTHROPIC_API_KEY not set"),
    )
    .build();

    let mut prompt_file = File::open(&std::env::var("PROMPT_FILE_PATH")?)?;
    let mut contents = String::new();
    prompt_file.read_to_string(&mut contents)?;

    // Create agent with a single context prompt
    let agent = client
        .agent(CLAUDE_3_5_SONNET)
        .preamble(&contents)
        .temperature(0.5)
        .max_tokens(8192)
        .build();

    let app_state = Arc::new(AppState { agent });

    let app = Router::new()
        .route("/", routing::get(index))
        .route("/prompt", routing::any(prompt))
        // Add some logging so we can see the streams going through
        .layer(TraceLayer::new_for_http().on_body_chunk(
            |chunk: &Bytes, _latency: Duration, _span: &Span| {
                tracing::debug!("streaming {} bytes", chunk.len());
            },
        ))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

// Include utf-8 file at **compile** time.
async fn index() -> String {
    String::from("Hello, World!")
}

async fn prompt(
    State(state): State<Arc<AppState>>,
    Json(message): Json<MessageRequest>,
) -> Result<impl IntoResponse, AppError> {
    println!("{message:?}");
    // Prompt the agent and print the response
    // let response = state.agent.prompt("What is a NCN?").await?;

    Ok(Json("Hello"))
}
