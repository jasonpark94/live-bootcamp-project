use axum::{extract::State, http::StatusCode, Json};
use serde::Deserialize;

use crate::{app_state::{self, AppState}, domain::AuthAPIError, utils::auth::validate_token};


pub async fn verify_token(
    State(state): State<AppState>,
    Json(request): Json<VerifyTokenRequest>,
) -> Result<StatusCode, AuthAPIError> {

    let token = request.token;
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