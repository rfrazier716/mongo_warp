use super::*;

pub async fn health_handler(db: database::Database) -> Result<impl Reply, Rejection> {
    db.ping()
        .await
        .map_err(|e| error::ServerError::DataBaseError { source: e })?;
    Ok(StatusCode::OK)
}
