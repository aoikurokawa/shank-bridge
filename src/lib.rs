use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
pub struct MessageRequest {
    messages: Vec<Message>,
}
