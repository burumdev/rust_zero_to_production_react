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

#[tokio::test]
async fn it_doesnt_deliver_newsletters_to_unconfirmed_subscribers() {
    let test_app = spawn_app().await;

    create_unconfirmed_subscriber(&test_app).await;

    let newsletter_body = serde_json::json!({
        "title": "Test Newsletter Title",
        "content": {
            "text": "Newsletter body.",
            "html": "<p>Newsletter body as HTML</p>",
        }
    });

    let response = reqwest::Client::new()
        .post(&format!("{}/api/newsletters", &test_app.address))
        .json(&newsletter_body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn it_delivers_newsletters_to_confirmed_subscribers() {
    let test_app = spawn_app().await;

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

    for link in link_finder.links(&email_txt) {
        if link.as_str().contains("/confirm?") {
            reqwest::Client::new()
                .post(link.as_str())
                .send()
                .await
                .expect("Failed To execute request.");

            let newsletter_body = serde_json::json!({
                "title": "Test Newsletter Title",
                "content": {
                    "text": "Newsletter body.",
                    "html": "<p>Newsletter body as HTML</p>",
                }
            });

            let response = reqwest::Client::new()
                .post(&format!("{}/api/newsletters", &test_app.address))
                .json(&newsletter_body)
                .send()
                .await
                .expect("Failed to execute request.");

            assert_eq!(response.status().as_u16(), 200);
        } else {
            panic!("Email confirm link was malformed!");
        }
    }
}
