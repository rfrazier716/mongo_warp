use crate::database;
use crate::error;
use std::convert::Infallible;
use warp::http::StatusCode;
use warp::{reject, Filter, Rejection, Reply};

pub mod health_check;

pub fn with_db(
    db: database::Database,
) -> impl Filter<Extract = (database::Database,), Error = Infallible> + Clone {
    warp::any().map(move || db.clone())
}
