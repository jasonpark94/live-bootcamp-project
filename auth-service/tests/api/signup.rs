use crate::helpers::{get_random_email, TestApp};
use auth_service::{routes::SignupResponse, ErrorResponse};
use axum::http::response;   

#[tokio::test]
async fn should_return_422_if_malformed_input() {
    let app = TestApp::new().await;

    let random_email = get_random_email(); // Call helper method to generate email 

    // TODO: add more malformed input test cases
    let test_cases = [
        serde_json::json!({
            "password": "password123",
            "requires2Fa": true
        }),
        serde_json::json!({
            "email": random_email,
            "password": "password123",
        }),
    ];

    for test_case in test_cases.iter() {
        let response = app.post_signup(&test_case).await; // call `post_signup`
        assert_eq!(
            response.status().as_u16(),
            422,
            "Failed for input: {:?}",
            test_case
        );
    }
}

#[tokio::test]
async fn should_return_201_if_valid_input() {
    let app = TestApp::new().await;
    let test_cases = [
        serde_json::json!({
        "email": get_random_email(),
        "password": "password",
        "requires2FA": true,
        })];

    let expected_response = SignupResponse {
        message: "User created successfully!".to_owned(),
    };

    
    for test_case in test_cases.iter() {
        let response = app.post_signup(&test_case).await;

        assert_eq!(
            response
                .json::<SignupResponse>()
                .await
                .expect("Could not deserialize response body to UserBody"),
            expected_response
        );
    }

}

#[tokio::test]
async fn should_return_400_if_invalid_input() {
    let app = TestApp::new().await;
    let test_cases = [
        serde_json::json!({
            "email": "testemail", 
            "password": "password",
            "requires2FA": true,
        }),
        serde_json::json!({
            "email": "testemail", 
            "password": "passwor",
            "requires2FA": true,
        }
    )];

    for i in test_cases.iter() {
        let response = app.post_signup(i).await;
        assert_eq!(response.status().as_u16(), 400, "Failed for input: {:?}", i);

        assert_eq!(
            response
                .json::<ErrorResponse>()
                .await
                .expect("Could not deserialize response body to ErrorResponse")
                .error,
            "Invalid credentials".to_owned()
        );
    }
}

#[tokio::test]
async fn should_return_409_if_email_already_exists() {
    let app = TestApp::new().await;

    let test_user = serde_json::json!({
        "email": "test1@test.com",
        "password": "password",
        "requires2FA": true,
    });

    app.post_signup(&test_user).await;

    let test_case = 
        serde_json::json!({
            "email": "test1@test.com", 
            "password": "password",
            "requires2FA": true,
        });

    let response= app.post_signup(&test_case).await;

    assert_eq!(response.status().as_u16(), 409);

    assert_eq!(
        response
            .json::<ErrorResponse>()
            .await
            .expect("Could not deserialize response body to ErrorResponse")
            .error,
        "User already exists".to_owned()
    );
}