use std::net::TcpListener;
use zero_to_prod::launch_server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    launch_server(listener)?.await // start the server as an async function
}
