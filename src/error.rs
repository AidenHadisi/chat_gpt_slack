#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("request to ChatGPT api failed -- {0}")]
    ApiError(String),

    #[error("slack error -- {0}")]
    SlackError(String),
}

pub type Result<T> = std::result::Result<T, Error>;