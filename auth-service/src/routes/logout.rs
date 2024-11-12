use axum::{extract::State, http::StatusCode, response::IntoResponse};
use axum_extra::extract::CookieJar;

use crate::{
    app_state::AppState, domain::{AuthAPIError, BannedTokenStore, BannedTokenStoreError}, utils::{auth::validate_token, constants::JWT_COOKIE_NAME}
};

pub async fn logout(jar: CookieJar, State(state): State<AppState>) -> (CookieJar, Result<impl IntoResponse, AuthAPIError>) {
    let cookie = match jar.get(JWT_COOKIE_NAME) {
        Some(cookie) => cookie,
        None => return (jar, Err(AuthAPIError::MissingToken)), 
    };

    let token = cookie.value().to_owned();

    match validate_token(&token).await {
        Ok(_) => {},
        Err(_) => return (jar, Err(AuthAPIError::InvalidToken)),
    };

    state.banned_token_store.write().await.add_banned_token(token.clone()).await;

    let jar = jar.remove(JWT_COOKIE_NAME);
    (jar, Ok(StatusCode::OK))
}