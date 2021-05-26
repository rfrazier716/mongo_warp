use crate::{database, routes};
use std::net::{SocketAddr, TcpListener};
use warp::{reply::Reply, Filter, Server};

pub fn run(listener: TcpListener, database: database::Database) -> impl warp::Future {
    // Create the Health Check Endpoint
    let health = warp::path!("health")
        .and(routes::with_db(database.clone()))
        .and_then(routes::health_check::health_handler);

    //Generate a future for the server
    warp::serve(health).serve_incoming(listener)
}
