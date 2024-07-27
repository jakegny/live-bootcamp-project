use crate::helpers::TestApp;

#[tokio::test]
async fn login_exists() {
    let app = TestApp::new().await;

    let response = app.get_login().await;

    assert_eq!(response.status().as_u16(), 200);
}
