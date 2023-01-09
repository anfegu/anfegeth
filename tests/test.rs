use dotenv::dotenv;
#[tokio::test]
async fn test_http_connection() {
    dotenv().ok();
    let url = std::env::var("URL").expect("URL must be set within .env file");
    let client = reqwest::Client::new();
    let resp = client.get(url).send().await.unwrap();
    assert_eq!(resp.status(), reqwest::StatusCode::OK);
}
