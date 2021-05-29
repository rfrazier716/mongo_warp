mod common;

#[tokio::test]
async fn test_health_check_endpoint() {
    //spawn the app so the server is running
    let app_address = common::spawn_app().unwrap();
    let client = reqwest::Client::new();

    // Create the health endpoint and set a get request to the health endpoint
    let endpoint = format!("http://{}:{}/health", app_address.ip(), app_address.port());
    let resp = client.get(endpoint).send().await.unwrap();

    // assert that we got a "success" error code back
    assert!(resp.status().is_success());
}
