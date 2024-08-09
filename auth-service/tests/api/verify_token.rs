use crate::helpers::{get_random_email, TestApp};
use auth_service::utils::constants::JWT_COOKIE_NAME;

#[tokio::test]
async fn should_return_422_if_malformed_input() {
    let app = TestApp::new().await;
    let invalid_body = serde_json::json!({});

    let response = app.post_verify_token(&invalid_body).await;

    assert_eq!(response.status().as_u16(), 422);
}

#[tokio::test]
async fn should_return_200_valid_token() {
    let app = TestApp::new().await;

    let random_email = get_random_email();
    let password = "password123";
    let new_user = serde_json::json!({
        "email": random_email,
        "password": password,
        "requires2FA": false,
    });
    app.post_signup(&new_user).await;

    let user_credentials = serde_json::json!({
        "email": random_email,
        "password": password,
    });
    let login = app.post_login(&user_credentials).await;

    let auth_cookie = login
        .cookies()
        .find(|cookie| cookie.name() == JWT_COOKIE_NAME)
        .expect("No auth cookie found");

    let token = auth_cookie.value();
    let response = app
        .post_verify_token(&serde_json::json!({ "token": token }))
        .await;
    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn should_return_401_if_invalid_token() {
    let app = TestApp::new().await;

    let response = app
        .post_verify_token(&serde_json::json!({ "token": "invalid-token".to_owned() }))
        .await;
    assert_eq!(response.status().as_u16(), 401);
}
