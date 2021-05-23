use mongodb::options::{ClientOptions, Credential, ServerAddress};
use mongodb::Client;
use std::net::TcpListener;
use zero_to_prod::launch_server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    let credentials = Credential::builder()
        .username("root".to_string())
        .password("example".to_string())
        .build();

    let db_options = ClientOptions::builder()
        .hosts(vec![ServerAddress::Tcp {
            host: "127.0.0.1".into(),
            port: Some(27017),
        }])
        .credential(credentials)
        .build();

    // Get a handle to the deployment.
    let client = Client::with_options(db_options).expect("Could not Connect to Client");

    launch_server(listener, client)?.await // start the server as an async function
}
