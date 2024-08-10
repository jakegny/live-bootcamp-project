use std::sync::Arc;
use tokio::sync::RwLock;

use super::hashset_banned_token_store::HashSetBannedTokenStore;
use crate::services::hashmap_user_store::HashmapUserStore;

pub type UserStoreType = Arc<RwLock<HashmapUserStore>>;
pub type BannedTokenStoreType = Arc<RwLock<HashSetBannedTokenStore>>;

#[derive(Clone)]
pub struct AppState {
    pub user_store: UserStoreType,
    pub banned_token_store: BannedTokenStoreType,
}

impl AppState {
    pub fn new(user_store: UserStoreType, banned_token_store: BannedTokenStoreType) -> Self {
        Self {
            user_store,
            banned_token_store,
        }
    }
}
