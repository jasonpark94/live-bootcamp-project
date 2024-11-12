use auth_service::{
    app_state::AppState, services::hashmap_user_store::HashmapUserStore, utils::constants::prod,
    Application,
};

#[tokio::main]
async fn main() {
    let user_store = auth_service::services::HashmapUserStore::default();
    let user_store = std::sync::Arc::new(tokio::sync::RwLock::new(user_store));

    let banned_token_store = auth_service::services::HashsetBannedTokenStore::default();
    let banned_token_store = std::sync::Arc::new(tokio::sync::RwLock::new(banned_token_store));

    let app_state = auth_service::app_state::AppState { user_store, banned_token_store };

    let app = Application::build(app_state, prod::APP_ADDRESS)
        .await
        .expect("Failed to build app");

    app.run().await.expect("Failed to run app");
}
