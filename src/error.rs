use actix_web;
use mongodb;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServerError {
    #[error("Database Error: {source}")]
    DataBaseError { source: mongodb::error::Error },
}

impl actix_web::error::ResponseError for ServerError {}
