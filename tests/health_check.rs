#[tokio::test]
async fn health_check_works(){
    // spawn_app().wait.expect("Failed to spawn our app.");
    spawn_app();

    // bring in reqwest to perform HTTP requests on application
    let client = reqwest::Client::new();

    // Act
    let response = client

    .get("http://127.0.0.1:8000/health_check")
    .send()
    .await
    .expect("Failed to execute request.");

    // assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

 fn spawn_app() {
    let server = zero2prod::run().expect("Failed to bind address");
    // launch server as background task
    // tokio::spawn returns handle to spawned future but no use for it
    // so there is non-binding let
    let _ = tokio::spawn(server);
}

