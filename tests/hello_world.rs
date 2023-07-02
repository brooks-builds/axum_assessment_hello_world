#[tokio::test]
async fn hello_world() {
    let response = reqwest::get("http://localhost:3000").await.unwrap();
    assert_eq!(response.text().await.unwrap(), "hello world");
}
