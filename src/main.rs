use tracing_subscriber::fmt::format::FmtSpan;
use {{crate_name}}::{config, startup};

#[tokio::main]
async fn main() {
    // load the config
    println!("Spawning Server");
    let server_config = config::Settings::new().expect("Could not Load Server Configuration");

    // Load the Trace filter settings from the Configuration
    // We're slicing the struct apart but that should be fine since we don't need that field anymore
    let filter = server_config.log.join(",");

    tracing_subscriber::fmt()
        .with_env_filter(filter) // Use the filter from the Config file
        // Record an event when each span closes. Used to time duration of spans
        .with_span_events(FmtSpan::CLOSE)
        .init(); // initialize the subscriber

    // Start the Server
    let (address, server) = startup::run(server_config)
        .await
        .expect("Could not Initialize Server");
    println!("Server running on Address: {:?}", address);
    server.await;
}
