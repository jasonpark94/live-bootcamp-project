use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use axum_extra::extract::CookieJar;
use serde::{Deserialize, Serialize};

use crate::{
    app_state::AppState,
    domain::{AuthAPIError, Email, LoginAttemptId, Password, TwoFACode},
    utils::auth::{self, generate_auth_cookie},
};

pub async fn login(
    State(state): State<AppState>,
    jar: CookieJar, // New!
    Json(request): Json<LoginRequest>,
) -> (CookieJar, Result<impl IntoResponse, AuthAPIError>) {

    let user_store = state.user_store.read().await;

    let email = match Email::parse(request.email) {
        Ok(email) => email,
        Err(_) => return (jar, Err(AuthAPIError::InvalidCredentials)),
    };

    let password = match Password::parse(request.password) {
        Ok(password) => password,
        Err(_) => return (jar, Err(AuthAPIError::InvalidCredentials)),
    };

    if let Err(_) = user_store.validate_user(&email, &password).await {
        return (jar, Err(AuthAPIError::UnauthorizedCredentials));
    }


    let user = match user_store.get_user(&email).await {
        Ok(user) => user,
        Err(_) => return (jar, Err(AuthAPIError::IncorrectCredentials)),
    };

    match user.requires_2fa {
        true => handle_2fa(&user.email, &state, jar).await,
        false => handle_no_2fa(&user.email, jar).await,
    }

    // ---
 //   let updated_jar = jar.add(auth_cookie);
//    (updated_jar, Ok(StatusCode::OK.into_response()))
}

#[derive(Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(tag = "type")]
pub enum LoginResponse {
    RegularAuth,
    TwoFactorAuth(TwoFactorAuthResponse),
}

pub struct RegularAuth {
    pub message: String,
}

// If a user requires 2FA, this JSON body should be returned!
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct TwoFactorAuthResponse {
    pub message: String,
    #[serde(rename = "loginAttemptId")]
    pub login_attempt_id: String,
}

async fn handle_2fa(
    email: &Email,
    state: &AppState,
    jar: CookieJar,
) -> (
    CookieJar,
    Result<(StatusCode, Json<LoginResponse>), AuthAPIError>,
) {
    let login_attempt_id = LoginAttemptId::default();
    let two_fa_code = TwoFACode::default();

    if state
        .two_fa_code_store
        .write()
        .await
        .add_code(email.clone(), login_attempt_id.clone(), two_fa_code.clone())
        .await
        .is_err()
    {
        return (jar, Err(AuthAPIError::Unexpected));
    }

    if state
        .email_client
        .send_email(email, "Two FA Code", &format!("Your 2FA code is {:?}", two_fa_code))
        .await
        .is_err()
    {
        return (jar, Err(AuthAPIError::Unexpected));
    }


    let response = Json(LoginResponse::TwoFactorAuth(TwoFactorAuthResponse {
        message: "2FA required".to_owned(),
        login_attempt_id: login_attempt_id.as_ref().to_owned(),
    }));

    let auth_cookie = generate_auth_cookie(&email).unwrap();
    let updated_jar = jar.add(auth_cookie);

    (updated_jar, Ok((StatusCode::PARTIAL_CONTENT, response)))
}

async fn handle_no_2fa(
    email: &Email,
    jar: CookieJar,
) -> (
    CookieJar,
    Result<(StatusCode, Json<LoginResponse>), AuthAPIError>,
) {
    let auth_cookie = generate_auth_cookie(&email).unwrap();
    let updated_jar = jar.add(auth_cookie);
    (updated_jar, Ok((StatusCode::OK, Json(LoginResponse::RegularAuth))))

}

