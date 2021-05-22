use actix_rt;
use reqwest;
use std::net::{SocketAddr, TcpListener};
use zero_to_prod::launch_server;

#[actix_rt::test]
async fn health_check_running() {
    //start the app and get the address
    let addr = spawn_app();

    // create a client and send a get request to the Health Check URL
    let client = reqwest::Client::new();
    let response = client
        .get(format!("http://{}:{}/health_check", addr.ip(), addr.port()))
        .send()
        .await
        .expect("Failed to Trigger Request");

    // verify request succeeded
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() -> SocketAddr {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Could not Create Listener");
    let addr = listener.local_addr().expect("Could not resolve IP Address");
    let server = launch_server(listener).expect("Failed to Launch Server");
    let _ = tokio::spawn(server);
    addr // return teh address
}
