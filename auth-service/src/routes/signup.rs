use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

use crate::{
    app_state::AppState,
    domain::{AuthAPIError, Email, Password, User, UserStore},
};

pub async fn signup(
    State(state): State<AppState>,
    Json(request): Json<SignupRequest>,
) -> Result<impl IntoResponse, AuthAPIError> {
    let email = match Email::parse(&request.email) {
        Ok(email) => email,
        Err(_) => return Err(AuthAPIError::InvalidCredentials),
    };
    let password = match Password::parse(&request.password) {
        Ok(password) => password,
        Err(_) => return Err(AuthAPIError::InvalidCredentials),
    };

    if !email.as_ref().contains('@') || email.as_ref().is_empty() {
        return Err(AuthAPIError::InvalidCredentials);
    }

    let user = User::new(email.as_ref(), password.as_ref(), request.requires_2fa);

    let mut user_store = state.user_store.write().await;

    if user_store.get_user(email).await.is_ok() {
        return Err(AuthAPIError::UserAlreadyExists);
    }

    if user_store.add_user(user).await.is_err() {
        return Err(AuthAPIError::UnexpectedError);
    }

    let response = Json(SignupResponse {
        message: "User created successfully!".to_string(),
    });

    Ok((StatusCode::CREATED, response))
}

#[derive(Deserialize)]
pub struct SignupRequest {
    pub email: String,
    pub password: String,
    #[serde(rename = "requires2FA")]
    pub requires_2fa: bool,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct SignupResponse {
    pub message: String,
}