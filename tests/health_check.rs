//! tests/health_check.rs

use std::net::TcpListener;

#[tokio::test]
async fn health_check_works() {
    let url = spawn_app();

    let client = reqwest::Client::new();

    let response = client
        .get(format!("{url}/health_check"))
        .send()
        .await
        .expect("failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("localhost:0").expect("Failed to bind to random port");
    let port = listener.local_addr().unwrap().port();

    let server = zero2prod::run(listener).expect("Failed to bind address");

    let _ = tokio::spawn(server);

    format!("http://localhost:{port}")
}
