use uuid::Uuid;
use zero2prod::domain::common::SuccessResponse;

use crate::helpers::spawn_app;

#[tokio::test]
async fn it_clears_session_with_logout() {
    let test_app = spawn_app().await;

    let login_json = serde_json::json!({
        "username": &test_app.test_user.username,
        "password": &test_app.test_user.password
    });

    let response = test_app.post_login(&login_json.to_string()).await;

    assert_eq!(response.status().as_u16(), 200);
    let body_text = response.text().await.unwrap();
    let json: SuccessResponse = serde_json::from_str(&body_text).unwrap();
    assert!(json.success);

    let response = test_app.post_logout().await;
    assert_eq!(response.status().as_u16(), 200);

    let change_pass_json = serde_json::json!({
        "current_password": &test_app.test_user.password,
        "new_password": "somepassword",
        "new_password_check": "somepassword"
    });
    let response = test_app
        .post_change_password(&change_pass_json.to_string())
        .await;

    assert_eq!(401, response.status().as_u16());
}

#[tokio::test]
async fn it_checks_password_fields_match_properly() {
    let test_app = spawn_app().await;
    let new_pass = Uuid::new_v4().to_string();
    let new_pass_check = Uuid::new_v4().to_string();

    let login_json = serde_json::json!({
        "username": &test_app.test_user.username,
        "password": &test_app.test_user.password
    });

    test_app.post_login(&login_json.to_string()).await;

    let change_pass_json = serde_json::json!({
        "current_password": &test_app.test_user.password,
        "new_password": new_pass,
        "new_password_check": new_pass_check
    });

    let response = test_app
        .post_change_password(&change_pass_json.to_string())
        .await;
    assert_eq!(400, response.status().as_u16());

    let body_text = response.text().await.unwrap();
    let json: SuccessResponse = serde_json::from_str(&body_text).unwrap();
    assert!(!json.success);
    assert!(json.message.to_lowercase().contains("should match"));
}

#[tokio::test]
async fn it_changes_password_successfully() {
    let test_app = spawn_app().await;
    let new_pass = Uuid::new_v4().to_string();
    let new_pass_check = new_pass.clone();

    let login_json = serde_json::json!({
        "username": &test_app.test_user.username,
        "password": &test_app.test_user.password
    });

    test_app.post_login(&login_json.to_string()).await;

    let change_pass_json = serde_json::json!({
        "current_password": &test_app.test_user.password,
        "new_password": new_pass,
        "new_password_check": new_pass_check
    });

    let response = test_app
        .post_change_password(&change_pass_json.to_string())
        .await;

    assert_eq!(200, response.status().as_u16());
}
