use crate::helpers::{get_random_email, TestApp};
use auth_service::{
    domain::{Email, LoginAttemptId, TwoFACode, TwoFACodeStore},
    ErrorResponse,
};

#[tokio::test]
async fn should_return_422_if_malformed_input() {
    let app = TestApp::new().await;

    let random_email = get_random_email();

    let test_cases = [
        serde_json::json!({
            "wrong": "wrong value",
        }),
        serde_json::json!({
            "email": random_email,
        }),
        serde_json::json!({}),
    ];

    for test_case in test_cases {
        let response = app.post_verify_2fa(&test_case).await;

        assert_eq!(
            response.status().as_u16(),
            422,
            "Failed for input: {:?}",
            test_case
        );
    }
}

#[tokio::test]
async fn should_return_400_if_invalid_input() {
    let app = TestApp::new().await;

    let random_email = get_random_email();

    let test_cases = [
        serde_json::json!({
            "email": "invalid",
            "loginAttemptId": "invalid",
            "2FACode": "123456"
        }),
    ];

    for test_case in test_cases {
        let response = app.post_verify_2fa(&test_case).await;

        assert_eq!(
            response.status().as_u16(),
            400,
            "Failed for input: {:?}",
            test_case
        );
    }
}

#[tokio::test]
async fn should_return_401_if_same_code_twice() {    
    let app = TestApp::new().await;

    let random_email = get_random_email();

    let signup_body = serde_json::json!({
        "email": &random_email,
        "password": "password",
        "requires2FA": true
    });

    let response = app.post_signup(&signup_body).await;

    assert_eq!(response.status().as_u16(), 201);

    let login_body = serde_json::json!({
        "email": &random_email,
        "password": "password",
    }); 

    let response = app.post_login(&login_body).await;

    assert_eq!(response.status().as_u16(), 206);

    let email = Email::parse(random_email.clone()).unwrap();
    let (login_attempt_id, two_fa_code) = app.two_fa_code_store.read().await.get_code(&email).await.unwrap();

    let test_case = 
        serde_json::json!({
            "email": &random_email,
            "loginAttemptId": login_attempt_id.as_ref(),
            "2FACode": two_fa_code.as_ref()
        });
    let response = app.post_verify_2fa(&test_case).await;
    assert_eq!(
        response.status().as_u16(),
        200,
    );
    let response = app.post_verify_2fa(&test_case).await;
    assert_eq!(
        response.status().as_u16(),
        401,
        "Failed for input: {:?}",
        test_case
    );

}