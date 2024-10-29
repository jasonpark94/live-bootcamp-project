use crate::helpers::TestApp;

#[tokio::test]
pub async fn check_verify_token() {
    let app = TestApp::new().await;
    let response = app.post_verify_token().await;

    assert_eq!(response.status().as_u16(), 200);
}