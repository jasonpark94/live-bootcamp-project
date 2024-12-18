use std::sync::Arc;

use auth_service::{
    app_state::AppState, services::hashmap_user_store::HashmapUserStore, utils::constants::test,
    Application,
};
use tokio::sync::RwLock;
use uuid::Uuid;

use reqwest::cookie::Jar;
pub struct TestApp {
    pub address: String,
    pub cookie_jar: Arc<Jar>,
    pub two_fa_code_store: Arc<RwLock<auth_service::services::HashmapTwoFACodeStore>>,
    pub banned_token_store: Arc<RwLock<auth_service::services::HashsetBannedTokenStore>>,
    pub http_client: reqwest::Client,
}

impl TestApp {
    pub async fn new() -> Self {
        let user_store = auth_service::services::HashmapUserStore::default();
        let user_store = Arc::new(RwLock::new(user_store));
        let banned_token_store = auth_service::services::HashsetBannedTokenStore::default();
        let banned_token_store= Arc::new(RwLock::new(banned_token_store));
        let two_fa_code_store= auth_service::services::HashmapTwoFACodeStore::default();
        let two_fa_code_store= Arc::new(RwLock::new(two_fa_code_store));
        let email_client = Arc::new(auth_service::services::MockEmailClient);

        let app_state = AppState {
            user_store,
            banned_token_store: banned_token_store.clone(),
            two_fa_code_store: two_fa_code_store.clone(),
            email_client,
        };
        let app = Application::build(app_state, "127.0.0.1:0")
            .await
            .expect("Failed to build app");

        let address = format!("http://{}", app.address.clone());

        // Run the auth service in a separate async task
        // to avoid blocking the main test thread.
        #[allow(clippy::let_underscore_future)]
        let _ = tokio::spawn(app.run());

        let cookie_jar = Arc::new(Jar::default());
        let http_client = reqwest::Client::builder()
            .cookie_provider(cookie_jar.clone())
            .build()
            .unwrap();

        Self {
            address,
            cookie_jar,
            two_fa_code_store,
            banned_token_store,
            http_client,
        }
    }

    pub async fn get_root(&self) -> reqwest::Response {
        self.http_client
            .get(&format!("{}/", &self.address))
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn post_login<Body>(&self, body: &Body) -> reqwest::Response
    where
        Body: serde::Serialize,
    {
        self.http_client
            .post(&format!("{}/login", &self.address))
            .json(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn post_logout(&self) -> reqwest::Response {
        self.http_client
            .post(&format!("{}/logout", &self.address))
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn post_verify_2fa<Body>(&self, body: &Body) -> reqwest::Response
    where
        Body: serde::Serialize,
    {
        self.http_client
            .post(format!("{}/verify-2fa", &self.address))
            .json(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn post_signup<Body>(&self, body: &Body) -> reqwest::Response
    where
        Body: serde::Serialize,
    {
        self.http_client
            .post(&format!("{}/signup", &self.address))
            .json(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn post_verify_token<Body>(&self, body: &Body) -> reqwest::Response
    where
        Body: serde::Serialize,
    {
        self.http_client
            .post(format!("{}/verify-token", &self.address))
            .json(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }
}

pub fn get_random_email() -> String {
    format!("{}@example.com", Uuid::new_v4())
}
