use crate::helpers::TestApp;

#[tokio::test]
async fn verify_2fa_exists() {
    let app = TestApp::new().await;

    let response = app.get_verify_2fa().await;

    assert_eq!(response.status().as_u16(), 200);
}
