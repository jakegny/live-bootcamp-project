use auth_service::utils::constants::JWT_COOKIE_NAME;
use reqwest::Url;

use crate::helpers::{get_random_email, TestApp};

#[tokio::test]
async fn should_return_400_if_jwt_cookie_missing() {
    let app = TestApp::new().await;

    let response = app.post_logout().await;

    assert_eq!(response.status().as_u16(), 400);
}

#[tokio::test]
async fn should_return_401_if_invalid_token() {
    let app = TestApp::new().await;

    // add invalid cookie
    app.cookie_jar.add_cookie_str(
        &format!(
            "{}=invalid; HttpOnly; SameSite=Lax; Secure; Path=/",
            JWT_COOKIE_NAME
        ),
        &Url::parse("http://127.0.0.1").expect("Failed to parse URL"),
    );

    let response = app.post_logout().await;
    assert_eq!(response.status().as_u16(), 401);
}

#[tokio::test]
async fn should_return_200_if_valid_jwt_cookie() {
    let random_email = get_random_email();
    let password = "password123";
    let new_user = serde_json::json!({
        "email": random_email,
        "password": password,
        "requires2FA": false,
    });

    let user_credentials = serde_json::json!({
        "email": random_email,
        "password": password,
    });

    let app = TestApp::new().await;

    app.post_signup(&new_user).await;
    app.post_login(&user_credentials).await;

    let response = app.post_logout().await;

    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn should_return_400_if_logout_called_twice_in_a_row() {
    let random_email = get_random_email();
    let password = "password123";
    let new_user = serde_json::json!({
        "email": random_email,
        "password": password,
        "requires2FA": false,
    });

    let user_credentials = serde_json::json!({
        "email": random_email,
        "password": password,
    });

    let app = TestApp::new().await;

    app.post_signup(&new_user).await;
    app.post_login(&user_credentials).await;
    app.post_logout().await;
    let response = app.post_logout().await;

    assert_eq!(response.status().as_u16(), 400);
}
