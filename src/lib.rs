pub mod chat_gpt;
pub mod error;
pub use error::*;

pub mod prelude {
    pub use super::chat_gpt::api::ChatGPT;
    pub use super::chat_gpt::api::ChatGPTApi;
    pub use super::chat_gpt::auth::SessionToken;
    pub use super::Error::*;
}
