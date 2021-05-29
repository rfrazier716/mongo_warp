use std::net::SocketAddr;
use tokio;
use zero_to_prod::{config, error::Result, startup};

pub fn spawn_app() -> Result<(SocketAddr)> {
    std::env::set_var("RUN_ENV", "test"); //set the test environment so the right config is loaded
    let app_settings = config::Settings::new()?;
    let (addr, server) = startup::run(app_settings)?;
    tokio::task::spawn(server);
    Ok(addr) // Return the bound address
}
