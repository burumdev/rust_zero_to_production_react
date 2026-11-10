use crate::helpers::spawn_app;

#[tokio::test]
async fn it_rejects_login_attempts_with_wrong_credentials() {
    let test_app = spawn_app().await;

    let login_body = serde_json::json!({
        "username": "-",
        "password": "-",
    });
    let response = test_app.post_login(&login_body.to_string()).await;

    assert_eq!(response.status().as_u16(), 401);
}
