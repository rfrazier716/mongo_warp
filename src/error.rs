use thiserror::Error;
use warp::reject::Reject;

#[derive(Error, Debug)]
pub enum Error {

    #[error("Could not initialized Database client from URI: {0}")]
    ClientInitError(mongodb::error::Error),

    #[error("error during mongodb query: {0}")]
    MongoQueryError(mongodb::error::Error),

    #[error("Error Configuring Server: {0}")]
    ConfigurationError(config::ConfigError),
}

impl Reject for Error {}

pub type Result<T> = std::result::Result<T, Error>;
