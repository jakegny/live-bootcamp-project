use axum::{http::StatusCode, response::IntoResponse};
use axum_extra::extract::CookieJar;

use crate::{
    utils::{auth::validate_token, constants::JWT_COOKIE_NAME},
    AuthAPIError,
};

pub async fn logout(jar: CookieJar) -> (CookieJar, Result<impl IntoResponse, AuthAPIError>) {
    match jar.get(JWT_COOKIE_NAME) {
        Some(cookie) => {
            let token = cookie.value();
            match validate_token(token).await {
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
