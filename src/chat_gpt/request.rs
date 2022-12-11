use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    pub action: String,
    pub parent_message_id: String,
    pub model: String,
    pub messages: Vec<Messages>,
}

impl Request {
    pub fn new(prompt: String) -> Self {
        Self {
            messages: vec![Messages {
                content: Content {
                    parts: vec![prompt],
                    ..Default::default()
                },
                ..Default::default()
            }],
            ..Default::default()
        }
    }
}

impl Default for Request {
    fn default() -> Self {
        Self {
            action: "variant".to_string(),
            parent_message_id: Uuid::new_v4().to_string(),
            model: "text-davinci-002-render".to_string(),
            messages: vec![Default::default()],
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Messages {
    pub id: String,
    pub role: String,
    pub content: Content,
}

impl Default for Messages {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            role: "user".to_string(),
            content: Default::default(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Content {
    pub content_type: String,
    pub parts: Vec<String>,
}

impl Default for Content {
    fn default() -> Self {
        Self {
            content_type: "text".to_string(),
            parts: vec![],
        }
    }
}
