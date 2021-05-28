use super::*;

pub fn health_routes(
    db: database::Database,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    // Create the Health Check route
    warp::path!("health")
        .and(with_db(db))
        .and_then(health_handler)
        .with(warp::trace(|info| {
            // Construct our own custom span for this route.
            tracing::info_span!("Health Check", req.path = ?info.path())
        }))
}

pub async fn health_handler(db: database::Database) -> Result<impl Reply, Rejection> {
    tracing::info!("Pinging Database");
    db.ping()
        .await
        .map_err(|e| error::ServerError::DataBaseError { source: e })?;
    Ok(StatusCode::OK)
}
