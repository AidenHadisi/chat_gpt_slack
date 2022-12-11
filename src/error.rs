#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("request to api failed with status {0}")]
    ApiError(String),
}

pub type Result<T> = std::result::Result<T, Error>;