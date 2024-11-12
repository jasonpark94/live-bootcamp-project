use axum::{extract::State, http::StatusCode, Json};
use serde::Deserialize;

use crate::{app_state::{AppState}, domain::AuthAPIError, utils::auth::validate_token};


pub async fn verify_token(
    State(state): State<AppState>,
    Json(request): Json<VerifyTokenRequest>,
) -> Result<StatusCode, AuthAPIError> {

    let token = request.token;

    let is_banned_token = state.banned_token_store.read().await.is_token_banned(&token).await.unwrap();
    match is_banned_token {
        true => return Err(AuthAPIError::InvalidToken),
        false => {},
    }

    let result = validate_token(&token).await;

    match result {
        Ok(_) => Ok(StatusCode::OK),
        Err(_) => Err(AuthAPIError::InvalidToken),
    }
}

#[derive(Debug, Deserialize)]
pub struct VerifyTokenRequest {
    token: String,
} 