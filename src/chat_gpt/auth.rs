///OpenAI session token. Must be manually retrieved.
pub struct SessionToken(pub String);

impl SessionToken {
    pub fn new(token: impl ToString) -> Self {
        Self(token.to_string())
    }

    ///Returns a session token as bearer string for Authrozation header.
    pub fn bearer(&self) -> String {
        format!("Bearer {}", self.0)
    }
}
