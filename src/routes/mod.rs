use crate::database;
use crate::error;
use std::convert::Infallible;
use warp::http::StatusCode;
use warp::{Filter, Rejection, Reply};

pub mod health;

pub fn routes(
    db: database::Database,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    health::health_routes(db)
}

pub fn with_db(
    db: database::Database,
) -> impl Filter<Extract = (database::Database,), Error = Infallible> + Clone {
    warp::any().map(move || db.clone())
}
