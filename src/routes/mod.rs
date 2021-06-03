use crate::db;
use crate::error;
use std::convert::Infallible;
use warp::http::StatusCode;
use warp::{Filter, Rejection, Reply};

pub mod health;

pub fn routes(
    client: db::Client,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    health::health_routes(client)
}

pub fn with_db(
    client: db::Client,
) -> impl Filter<Extract = (db::Client,), Error = Infallible> + Clone {
    warp::any().map(move || client.clone())
}
