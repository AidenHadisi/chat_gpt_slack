///OpenAI session token. Must be manually retrieved.
pub struct SessionToken(pub String);

///BearerToken authroization header to send to ChatGPT.
pub type BearerToken = String;

impl SessionToken {
    pub fn new(token: impl ToString) -> Self {
        Self(token.to_string())
    }
}

///Converts session token to a BearerToken.
impl From<SessionToken> for BearerToken {
    fn from(s: SessionToken) -> Self {
        format!("Bearer {}", s.0)
    }
}
