use crate::{database, routes};
use warp::Filter;

pub fn run(database: database::Database) -> impl warp::Future {
    // Create the Health Check route
    let health = warp::path!("health")
        .and(routes::with_db(database))
        .and_then(routes::health_check::health_handler)
        .with(warp::trace(|info| {
            // Construct our own custom span for this route.
            tracing::info_span!("Health Check", req.path = ?info.path())
        }));

    // Add all our routes
    let routes = health.with(warp::trace::request());

    //Generate a future for the server
    warp::serve(routes).run(([127, 0, 0, 1], 3030))
}
