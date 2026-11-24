use linkify::{LinkFinder, LinkKind};
use zero2prod::{domain::common::SuccessResponse, email_client::EmailSuccessResponse};

use crate::helpers::{TestApp, spawn_app};

async fn create_unconfirmed_subscriber(test_app: &TestApp) -> reqwest::Response {
    // Create an unconfirmed (default) subscriber
    let body = "{\"name\":\"Chuck Norris\",\"email\":\"chuck.norris@gmail.com\"}";

    // We're unwrapping here because we don't test this functionality here.
    // Posting subscriptions and email sending should be working at this point.
    let response = test_app
        .post_subscriptions(body)
        .await
        .error_for_status()
        .unwrap();

    response
}

async fn create_confirmed_subscriber(test_app: &TestApp) -> reqwest::Response {
    // The response value is the response from email testing API
    // in the format: {success: bool, message_ids: [message_id]}
    // We then send another request to email testing api with the message_id
    // for the contents of the email in the inbox.
    let response = create_unconfirmed_subscriber(&test_app).await;
    let json: SuccessResponse = response.json().await.unwrap();
    let data: EmailSuccessResponse = serde_json::from_value(json.data.unwrap()).unwrap();
    let message_id = data.message_ids[0].clone();

    let email_txt = test_app
        .email_client
        .get_sent_message_txt(message_id)
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let mut link_finder = LinkFinder::new();
    link_finder.kinds(&[LinkKind::Url]);

    let maybe_link = link_finder.links(&email_txt).last();
    if let Some(link) = maybe_link
        && link.as_str().contains("/confirm?")
    {
        reqwest::Client::new()
            .get(link.as_str())
            .send()
            .await
            .expect("Failed To execute request.")
    } else {
        panic!("Email confirm link was malformed!");
    }
}

#[tokio::test]
async fn it_doesnt_deliver_newsletters_to_unconfirmed_subscribers() {
    let test_app = spawn_app().await;

    test_app.test_user.login(&test_app).await;

    let response = create_unconfirmed_subscriber(&test_app).await;
    assert_eq!(response.status().as_u16(), 200);

    let newsletter_body = serde_json::json!({
        "title": "Test Newsletter Title",
        "content": "Newsletter body.",
    });

    let response = test_app.post_newsletters(newsletter_body).await;

    // The request should return 200 as it will silently avoid sending
    // newsletters to unconfirmed subscribers.
    // To test correctly, check email inbox to see if any emails are sent to Chuck Norris
    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn it_delivers_newsletters_to_confirmed_subscribers() {
    let test_app = spawn_app().await;

    test_app.test_user.login(&test_app).await;

    let response = create_confirmed_subscriber(&test_app).await;
    assert_eq!(response.status().as_u16(), 200);

    let newsletter_body = serde_json::json!({
        "title": "Test Newsletter Title",
        "content": "Newsletter body.",
    });

    let response = test_app.post_newsletters(newsletter_body).await;

    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn it_returns_400_for_invalid_newsletter_data() {
    let test_app = spawn_app().await;
    test_app.test_user.login(&test_app).await;

    let test_cases = vec![
        (
            serde_json::json!({
                "content": "Newsletter body as plain text",
            }),
            "missing title",
        ),
        (
            serde_json::json!({"title": "Newsletter!"}),
            "missing content",
        ),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = test_app.post_newsletters(invalid_body).await;

        assert_eq!(
            response.status().as_u16(),
            400,
            "The API did not return 400 Bad Request when the payload was {}.",
            error_message
        )
    }
}

#[tokio::test]
async fn it_idempotently_handles_newsletter_creation() {
    let test_app = spawn_app().await;
    let response = create_confirmed_subscriber(&test_app).await;
    assert_eq!(response.status().as_u16(), 200);

    let newsletter_body = serde_json::json!({
        "title": "Newsletter Title",
        "content": "Newsletter content as plain text",
        "idempotency_key": &uuid::Uuid::new_v4().to_string()
    });

    let response = test_app.post_newsletters(newsletter_body.clone()).await;
    assert_eq!(response.status().as_u16(), 200);

    todo!();
    // let response = test_app.post_newsletters(newsletter_body).await;
    // assert_eq!(response.status().as_u16(), 200);
}
