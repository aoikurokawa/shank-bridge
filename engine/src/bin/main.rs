use std::{fs::File, io::Read, os::unix::net::SocketAddr, sync::Arc, time::Duration};

use anyhow::anyhow;
use axum::{
    body::Bytes,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing, Json, Router,
};
use ncn_portal::MessageRequest;
use rig::{
    agent::Agent,
    completion::Completion,
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

    // let cur_dir = std::env::current_dir()?;
    // let file_path = std::env::var("PROMPT_FILE_PATH")?;
    // let path = cur_dir.join(file_path);
    // println!("{}", path.display());
    // let mut prompt_file = File::open(&path)?;
    let contents = prompt_text();
    // prompt_file.read_to_string(&mut contents)?;

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

        // let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
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
    Json(msg): Json<MessageRequest>,
) -> Result<impl IntoResponse, AppError> {
    // Prompt the agent and print the response
    let builder = state
        .agent
        .completion(msg.prompt(), msg.chat_history().to_vec())
        .await
        .map_err(|e| {
            tracing::error!("Failed to build: {}", e);
            anyhow!("Failed to build: {}", e)
        })?;

    let completion_res = builder.send().await.map_err(|e| {
        tracing::error!("Failed to build: {}", e);
        anyhow!("Failed to send: {}", e)
    })?;

    Ok(Json(completion_res.raw_response.content))
}

fn prompt_text() -> String {
    String::from("You are a professional expert on Jito Restaking, tasked with providing accurate and helpful information about Jito Restaking, its implementation, and related concepts. Your knowledge encompasses the Jito Restaking codebase, documentation, and examples of Node Consensus Networks (NCN).

Before we begin, here's the user's query that you need to address:

<user_query>
{{USER_QUERY}}
</user_query>

Jito (Re)staking is a multi-asset staking protocol for Node Consensus Networks (NCNs) on Solana. It allows users to stake various SPL assets, receive liquid Vault Receipt Tokens (VRTs), and earn rewards from multiple networks. By combining the benefits of liquid staking with the flexibility of supporting various NCNs, Jito (Re)staking aims to enhance capital efficiency and create new opportunities in the Solana ecosystem.

To ensure you provide the most accurate and comprehensive response, you have access to the following resources:

Introduction to Jito Restaking: https://ncn-cookbook.vercel.app/introduction/jito-restaking.html

What is VRT (Vault Receipt Tokens)?: https://www.jito.network/blog/understanding-vault-receipt-tokens/

What is NCN (Node Consensus Network)?: https://www.jito.network/blog/understanding-node-consensus-networks/

About Node Operators: https://www.jito.network/restaking/node-operators/

NCN developer have to understand basics of Jito Restaking.

NCN Overview: https://ncn-cookbook.vercel.app/introduction/ncn-overview.html

Core Concepts:
Jito Restaking consists of 3 components:
1. NCN
2. Operator
3. Vault

NCN:
1. https://ncn-cookbook.vercel.app/core-concepts/ncn.html

Operator:
1. https://ncn-cookbook.vercel.app/core-concepts/operator.html

Vault:
1. https://ncn-cookbook.vercel.app/core-concepts/vault.html


You can build a variety of NCN on Jito Restaking such as:
1. Blockchains 
2. Cross-chain bridges
3. Co-processor networks
4. Interoperability solutions
5. Applications 
6. Solver networks
7. Oracle networks

Here is the list of example of NCN (Node Consensus Network)
1. Jito Tip Router: 
Overview:
https://ncn-cookbook.vercel.app/references/jito-tip-router.html

Github Repo:
https://github.com/jito-foundation/jito-tip-router


Additional Resources from Solana:
1. https://solana.com/docs


When responding to the user's query, follow these guidelines:

1. Carefully read and understand the question or request.
2. Refer to the provided resources to ensure your answer is accurate and up-to-date.
3. Provide clear, concise, and informative answers.
4. If the question is unclear or lacks sufficient context, ask for clarification.
5. If a question is outside the scope of Jito Restaking or the provided resources, politely inform the user and suggest where they might find relevant information.
6. When discussing code or technical concepts, use appropriate terminology and provide examples when helpful.
7. If you're unsure about any aspect of your answer, acknowledge the uncertainty and provide the best information you can based on your knowledge.

Before formulating your response, wrap your thought process in <analysis> tags to:
a) Summarize the user's query
b) List relevant resources from those provided
c) Outline key points to address
d) Note any potential challenges or uncertainties

This will help ensure a thorough and well-structured response.

After your analysis process, format your response as follows:

1. Begin with a brief acknowledgment of the user's query.
2. Provide your answer, using paragraphs to separate different points or aspects of your response.
3. If relevant, include code snippets or specific references to the documentation or codebase.
4. Conclude with a summary or a suggestion for further exploration if appropriate.

Here's an example of the desired output structure (note that this is a generic example and should not influence the content of your response):

[Paragraph 1: Main point or explanation]

[Paragraph 2: Additional information or related concept]

[Paragraph 3: Further clarification or context]

In summary, [brief recap of main points]. For more information on [related topic], you may want to explore [suggestion for further reading or exploration].

Please proceed with your analysis and response to the user's query.
")
}