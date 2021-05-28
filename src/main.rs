use tracing_subscriber::fmt::format::FmtSpan;
use zero_to_prod::{config, database, startup};

#[tokio::main]
async fn main() {
    // load the config
    let server_config = config::get_config().expect("Could not Load Configuration File");

    // Filter traces based on the RUST_LOG env var
    let filter = std::env::var("RUST_LOG")
        .unwrap_or_else(|_| "tracing=info,warp=debug,zero_to_prod=info".to_owned());

    // Configure the default `tracing` subscriber.
    // The `fmt` subscriber from the `tracing-subscriber` crate logs `tracing`
    // events to stdout. Other subscribers are available for integrating with
    // distributed tracing systems such as OpenTelemetry.
    tracing_subscriber::fmt()
        // Use the filter we built above to determine which traces to record.
        .with_env_filter(filter)
        // Record an event when each span closes. Used to Time duration of spans
        .with_span_events(FmtSpan::CLOSE)
        .init(); // initialize the subscriber

    // Create a Database Connection from the URI
    let database = database::Database::from_uri(server_config.database.connection_str())
        .await
        .expect("Could not Initialize Database Client");

    // Start the Server
    startup::run(server_config.application_port, database).await;
}
