use actix_rt;
use reqwest;
use zero_to_prod::launch_server;

#[actix_rt::test]
async fn health_check_running() {
    //start the app
    spawn_app();

    let client = reqwest::Client::new();
    let response = client
        .get("http://127.0.0.1:8080/health_check")
        .send()
        .await
        .expect("Failed to Trigger Request");

    // verify request succeeded
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() {
    let server = launch_server().expect("Failed to Launch Server");
    let _ = tokio::spawn(server);
}
