use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

use crate::{
    domain::{
        data_stores::UserStore, email::Email, error::AuthAPIError, password::Password, user::User,
    },
    services::app_state::AppState,
};

pub async fn signup(
    State(state): State<AppState>,
    Json(request): Json<SignupRequest>,
) -> Result<impl IntoResponse, AuthAPIError> {
    let email =
        Email::parse(request.email.as_str()).map_err(|_| AuthAPIError::InvalidCredentials)?;
    let password =
        Password::parse(request.password.as_str()).map_err(|_| AuthAPIError::InvalidCredentials)?;
    let user = User::new(email, password, request.requires_2fa);

    let mut user_store = state.user_store.write().await;

    if user_store.get_user(&user.email).await.is_ok() {
        return Err(AuthAPIError::UserAlreadyExists);
    }
    match user_store.add_user(user).await {
        Ok(_) => {
            let response = Json(SignupResponse {
                message: "User created successfully!".to_string(),
            });
            Ok((StatusCode::CREATED, response))
        }
        Err(_) => Err(AuthAPIError::UnexpectedError),
    }
}

#[derive(Deserialize)]
pub struct SignupRequest {
    pub email: String,
    pub password: String,
    #[serde(rename = "requires2FA")]
    pub requires_2fa: bool,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct SignupResponse {
    pub message: String,
}
