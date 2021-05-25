use std::net::TcpListener;
use zero_to_prod::database;
use zero_to_prod::launch_server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    let db_config = database::DatabaseConfig {
        address: "127.0.0.1".to_string(),
        port: 27017,
        username: Some("root".to_string()),
        password: Some("example".to_string()),
    };
    let database = database::Database::new(db_config);

    launch_server(listener, database)?.await // start the server as an async function
}
