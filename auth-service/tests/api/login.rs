use crate::helpers::TestApp;
use auth_service::ErrorResponse;

#[tokio::test]
async fn login_exists() {
    let app = TestApp::new().await;

    let test_case = serde_json::json!({
            "email": "test@email.com",
            "password": "12345678",
    });
    let response = app.post_login(&test_case).await;

    assert_eq!(response.status().as_u16(), 401);
}

#[tokio::test]
async fn should_return_422_if_malformed_credentials() {
    let app = TestApp::new().await;
    let email = "test@email.com";

    let test_cases = [
        serde_json::json!({
            "password": "12345678",
        }),
        serde_json::json!({
            "email": email,
        }),
    ];

    for test_case in test_cases {
        let response = app.post_login(&test_case).await;
        assert_eq!(
            response.status().as_u16(),
            422,
            "Failed for input: {:#?}",
            test_case
        );
    }
}

#[tokio::test]
async fn should_return_400_if_invalid_input() {
    // Call the log-in route with invalid credentials and assert that a
    // 400 HTTP status code is returned along with the appropriate error message.
    let test_cases = [
        serde_json::json!({
            "email": "noatsign.com",
            "password": "12345678",
        }),
        serde_json::json!({
            "email": "",
            "password": "12345678",
        }),
        serde_json::json!({
            "email": "test@email.com",
            "password": "1234",
        }),
    ];

    for test_case in test_cases {
        let app = TestApp::new().await;
        let response = app.post_login(&test_case).await;
        assert_eq!(
            response.status().as_u16(),
            400,
            "Failed for input: {:?}",
            test_case
        );

        assert_eq!(
            response
                .json::<ErrorResponse>()
                .await
                .expect("Could not deserialize response body to ErrorResponse")
                .error,
            "Invalid credentials".to_string()
        );
    }
}

#[tokio::test]
async fn should_return_401_if_incorrect_credentials() {
    // Call the log-in route with incorrect credentials and assert
    // that a 401 HTTP status code is returned along with the appropriate error message.
    let random_email = "test@email.com";
    let valid_test = serde_json::json!({
        "email": random_email,
        "password": "password123",
        "requires2FA": true,
    });

    let app = TestApp::new().await;

    let _response = app.post_signup(&valid_test).await;

    let invalid_credentials = serde_json::json!({
        "email": random_email,
        "password": "garbagio",
    });

    let response = app.post_login(&invalid_credentials).await;

    assert_eq!(response.status().as_u16(), 401);

    assert_eq!(
        response
            .json::<ErrorResponse>()
            .await
            .expect("Could not deserialize response body to ErrorResponse")
            .error,
        "Incorrect credentials".to_string()
    );
}
