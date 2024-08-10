use axum::{extract::State, http::StatusCode, response::IntoResponse};
use axum_extra::extract::CookieJar;

use crate::{
    utils::{auth::validate_token, constants::JWT_COOKIE_NAME},
    AppState, AuthAPIError,
};

pub async fn logout(
    State(state): State<AppState>,
    jar: CookieJar,
) -> (CookieJar, Result<impl IntoResponse, AuthAPIError>) {
    let banned_token_store = state.banned_token_store.clone();
    match jar.get(JWT_COOKIE_NAME) {
        Some(cookie) => {
            let token = cookie.value();
            match validate_token(token, banned_token_store).await {
                Ok(_) => {
                    let cookie_clone = cookie.clone().into_owned();
                    (jar.remove(cookie_clone), Ok(StatusCode::OK.into_response()))
                }
                Err(_) => (jar, Err(AuthAPIError::InvalidToken)),
            }
        }
        None => (jar, Err(AuthAPIError::MissingToken)),
    }
}
