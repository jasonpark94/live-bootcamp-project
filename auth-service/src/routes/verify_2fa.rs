use axum::{extract::State, http::{status, StatusCode}, response::IntoResponse, Json};
use serde::Deserialize;
use crate::{app_state::AppState, domain::{AuthAPIError, Email, LoginAttemptId, TwoFACode}};

pub async fn verify_2fa(
    State(state): State<AppState>,
    Json(request): Json<Verify2FARequest>,
) -> Result<impl IntoResponse, AuthAPIError> {
    let email = match Email::parse(request.email) {
        Ok(email) => { email },
        Err(_) => {
            return Err(AuthAPIError::InvalidCredentials);
        },
    };

    let (stored_login_attempt_id, stored_code) = match state.two_fa_code_store.read().await.get_code(&email).await {
        Ok(result) => result,
        Err(_) => {
            return Err(AuthAPIError::UnauthorizedCredentials);
        },
    };

    let login_attempt_id = LoginAttemptId::parse(request.loginAttemptId).unwrap();
    if login_attempt_id.ne(&stored_login_attempt_id) {
        return Err(AuthAPIError::UnauthorizedCredentials);
    }

    let two_fa_code = TwoFACode::parse(request.twoFACode).unwrap();
    if two_fa_code.ne(&stored_code) {
        return Err(AuthAPIError::UnauthorizedCredentials);
    }   

    let remove = state.two_fa_code_store.write().await.remove_code(&email).await.unwrap();

    Ok(StatusCode::OK.into_response())
}

#[derive(Deserialize)]
pub struct Verify2FARequest {
    email: String,
    loginAttemptId: String,
    #[serde(rename = "2FACode")]
    twoFACode: String
}