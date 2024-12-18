use axum::{
    http::{Method, StatusCode},
    response::{IntoResponse, Response},
    routing::post,
    serve::Serve,
    Json, Router,
};
use domain::AuthAPIError;
use serde::{Deserialize, Serialize};
use std::error::Error;
use tower_http::{cors::CorsLayer, services::ServeDir};
use routes::{login, logout, signup, verify_2fa, verify_token};
use utils::constants::JWT_COOKIE_NAME;


pub mod routes;
pub mod services;
pub mod domain;
pub mod app_state;
pub mod utils;

use app_state::AppState;

use std::sync::Arc;
use tokio::sync::RwLock;

use crate::services::hashmap_user_store::HashmapUserStore;

// Using a type alias to improve readability!
pub type UserStoreType = Arc<RwLock<HashmapUserStore>>;


// This struct encapsulates our application-related logic.
pub struct Application {
    server: Serve<Router, Router>,
    // address is exposed as a public field
    // so we have access to it in tests.
    pub address: String,
}

impl Application {
    pub async fn build(app_state: AppState, address: &str) -> Result<Self, Box<dyn Error>> {
        let allowed_origins = [
            "http://localhost:8000".parse()?,
            "http://68.183.97.66:8000".parse()?,
        ];

        let cors = CorsLayer::new()
            // Allow GET and POST requests
            .allow_methods([Method::GET, Method::POST])
            // Allow cookies to be included in requests
            .allow_credentials(true)
            .allow_origin(allowed_origins);

        let router = Router::new()
            .nest_service("/", ServeDir::new("assets"))
            .route("/signup", post(signup))
            .route("/login", post(login))
            .route("/verify-2fa", post(verify_2fa))
            .route("/logout", post(logout))
            .route("/verify-token", post(verify_token))
            .with_state(app_state)
            .layer(cors); 

        let listener = tokio::net::TcpListener::bind(address).await?;
        let address = listener.local_addr()?.to_string();
        let server = axum::serve(listener, router);

        // Create a new Application instance and return it

        return Ok(Application { server, address });
    }


    pub async fn run(self) -> Result<(), std::io::Error> {
        println!("listening on {}", &self.address);
        self.server.await
    }
}

#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
}

impl IntoResponse for AuthAPIError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthAPIError::UserAlreadyExists => (StatusCode::CONFLICT, "User already exists"), // 
            AuthAPIError::InvalidCredentials => (StatusCode::BAD_REQUEST, "Invalid credentials"), // 400
            AuthAPIError::IncorrectCredentials => (StatusCode::BAD_REQUEST, "Incorrect credentials"), // 400
            AuthAPIError::UnauthorizedCredentials => (StatusCode::UNAUTHORIZED, "Unauthorized request"), // 400
            AuthAPIError::MisinformedCredentials => (StatusCode::UNPROCESSABLE_ENTITY, "Misinformed credentials"), // 422
            AuthAPIError::MissingToken => (StatusCode::BAD_REQUEST, "Missing auth token"), // 422
            AuthAPIError::InvalidToken => (StatusCode::UNAUTHORIZED, "Invalid auth token"), // 422
            AuthAPIError::Unexpected => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Unexpected error") // 500
            }
        };
        let body = Json(ErrorResponse {
            error: error_message.to_string(),
        });
        (status, body).into_response()
    }
}

