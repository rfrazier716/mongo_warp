use crate::{config, database, error, routes};
use std::future::Future;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tokio::signal;
use tokio::sync::oneshot;
use warp::{Filter, Server};

// Run is in its own function so it can be started as a separate task for Integration Tests
pub fn run(
    settings: config::Settings,
) -> Result<(SocketAddr, impl Future<Output = ()> + 'static), error::ServerError> {
    // Create a Database Connection from the URI
    let database = database::Database::new(settings.database)
        .map_err(|e| error::ServerError::DataBaseError { source: e })?;

    // Add all our routes
    let routes = routes::routes(database).with(warp::trace::request());

    // Create a Socket to bind the server to
    let socket = SocketAddr::new(
        IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
        settings.application_port,
    );

    //Start the Server
    Ok(
        warp::serve(routes).bind_with_graceful_shutdown(socket, async {
            match signal::ctrl_c().await {
                Ok(_) => println!("Shutting Down Server"),
                Err(_) => println!("Error handling SIGINT"),
            }
        }),
    )
}
