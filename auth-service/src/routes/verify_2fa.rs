use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Deserialize;

use crate::domain::AuthAPIError;

pub async fn verify_2fa(Json(request): Json<Verify2FARequest>) -> Result<impl IntoResponse, AuthAPIError>{
    Ok(StatusCode::OK.into_response())
}

#[derive(Deserialize)]
pub struct Verify2FARequest {
    email: String,
    loginAttemptId: String,
    twoFACode: String
}
