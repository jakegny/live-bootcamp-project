use auth_service::{
    services::{
        app_state::AppState, hashmap_user_store::HashmapUserStore,
        hashset_banned_token_store::HashSetBannedTokenStore,
    },
    utils::constants::prod,
    Application,
};
use std::sync::Arc;
use tokio::sync::RwLock;

#[tokio::main]
async fn main() {
    let user_store: Arc<RwLock<HashmapUserStore>> =
        Arc::new(RwLock::new(HashmapUserStore::default()));
    let banned_token_store: Arc<RwLock<HashSetBannedTokenStore>> =
        Arc::new(RwLock::new(HashSetBannedTokenStore::default()));
    let app_state = AppState::new(user_store, banned_token_store);

    let app = Application::build(app_state, prod::APP_ADDRESS)
        .await
        .expect("Failed to build app");

    app.run().await.expect("Failed to run app");
}
