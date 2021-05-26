use mongodb;
use thiserror::Error;
use warp::reject::Reject;

#[derive(Error, Debug)]
pub enum ServerError {
    #[error("Database Error: {source}")]
    DataBaseError { source: mongodb::error::Error },
}

impl Reject for ServerError {}
