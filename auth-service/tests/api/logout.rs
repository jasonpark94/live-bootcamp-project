use crate::helpers::TestApp;

#[tokio::test]
pub async fn check_logout() {
    let app = TestApp::new().await;
    let response = app.post_logout().await;

    assert_eq!(response.status().as_u16(), 200);
}