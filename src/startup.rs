use crate::{config, database, error, routes};
use warp::Filter;

// Run is in its own function so it can be started as a separate task for Integration Tests
pub fn run(settings: config::Settings) -> Result<impl warp::Future, error::ServerError> {
    // Create a Database Connection from the URI
    let database = database::Database::new(settings.database)
        .map_err(|e| error::ServerError::DataBaseError { source: e })?;

    // Add all our routes
    let routes = routes::routes(database).with(warp::trace::request());

    //Generate a future for the server
    Ok(warp::serve(routes).run(([127, 0, 0, 1], settings.application_port)))
}
