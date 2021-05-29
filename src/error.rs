use thiserror::Error;
use warp::reject::Reject;

#[derive(Error, Debug)]
pub enum ServerError {
    #[error("Database Error: {source}")]
    DataBaseError { source: mongodb::error::Error },

    #[error("Error Configuring Server: {source}")]
    ConfigurationError { source: config::ConfigError },
}

impl Reject for ServerError {}

pub type Result<T> = std::result::Result<T, ServerError>;
