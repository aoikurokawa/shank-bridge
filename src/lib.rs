use rig::completion::Message;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct MessageRequest {
    chat_history: Vec<Message>,
    prompt: String,
}

impl MessageRequest {
    pub fn new(chat_history: &[Message], prompt: String) -> Self {
        Self {
            chat_history: chat_history.to_vec(),
            prompt,
        }
    }

    pub fn chat_history(&self) -> &[Message] {
        &self.chat_history
    }

    pub fn prompt(&self) -> &str {
        &self.prompt
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Content {
    text: String,
    content_type: String,
}

impl Content {
    pub fn new(text: String, content_type: String) -> Self {
        Self { text, content_type }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageResponse {
    content: Vec<Content>,
}

impl MessageResponse {
    pub fn new(content: &[Content]) -> Self {
        Self {
            content: content.to_vec(),
        }
    }
}
