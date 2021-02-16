/// Test that we can get a request and that 
/// it has no body.
#[actix_rt::test]
async fn health_check_works() {
    spawn_app();

    let client = reqwest::Client::new();
    
    let response = client
        .get("http://127.0.0.1:8000/health_check")
        .send()
        .await
        .expect("Failed the execute request!");

     assert!(response.status().is_success());
     assert_eq!(Some(0), response.content_length());
}

fn spawn_app() {
    let server = newsletter::run().expect("Failed to bind address!");
    let _ = tokio::spawn(server);
}