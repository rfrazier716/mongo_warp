use env_logger::Env;
use std::convert::Infallible;
use std::net::TcpListener;
use warp::http::StatusCode;
use warp::{reject, Filter, Rejection, Reply};
use zero_to_prod::database;
use zero_to_prod::error;
use zero_to_prod::routes;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Could Not Bind Port");
    let address = listener
        .local_addr()
        .expect("Could not get Address of Listener");
    let uri = "mongodb://root:example@localhost:27017".to_string();

    let database = database::Database::from_uri(uri)
        .await
        .expect("Could not Initialize Database Client");

    // Start the Server
    zero_to_prod::startup::run(listener, database).await;
}
