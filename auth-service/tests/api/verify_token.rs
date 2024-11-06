use auth_service::utils::auth::validate_token;
use serde_json::json;
use crate::helpers::{get_random_email, TestApp};

#[tokio::test]
async fn should_return_422_if_malformed_input() {
    let app = TestApp::new().await;

    let response = app.post_verify_token(&json!({
        "wrong_key": "wrong_value"
    })).await;

    assert_eq!(response.status().as_u16(), 422);
}

#[tokio::test]
async fn should_return_200_valid_token() {
    let app = TestApp::new().await;

    let email = get_random_email();

    let signup_response = app.post_signup(&json!({
        "email": email,
        "password": "password",
        "requires2FA": false
    })).await;

    let login_response = app.post_login(&json!({
        "email": email,
        "password": "password",
    })).await;

    let token = login_response.cookies().find(|c| c.name() == "jwt").unwrap().value().to_owned();

    let verify_token_response = app.post_verify_token(&json!({
        "token": &token
    })).await;

    assert_eq!(verify_token_response.status().as_u16(), 200);
}

#[tokio::test]
async fn should_return_401_if_invalid_token() {
    let app = TestApp::new().await;
    let response = app.post_verify_token(&json!({
        "token": "wrong_value"
    })).await;
    assert_eq!(response.status().as_u16(), 401);
}