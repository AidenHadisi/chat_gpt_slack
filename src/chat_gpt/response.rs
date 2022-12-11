use serde::{Deserialize, Serialize};

///Response returned from ChatGPT API.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Response {
    pub message: Message,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub id: String,
    pub role: String,
    pub content: Content,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Content {
    pub content_type: String,
    pub parts: Vec<String>,
}
