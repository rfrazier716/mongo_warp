use actix_web;
use mongodb;
use thiserror::Error;
use warp::reject::Reject;

#[derive(Error, Debug)]
pub enum ServerError {
    #[error("Database Error: {source}")]
    DataBaseError { source: mongodb::error::Error },
}

impl actix_web::error::ResponseError for ServerError {}
impl Reject for ServerError {}
