use parking_lot::RwLock;

use crate::env::Env;
use std::{io::Write, ops::Deref};

pub mod announce;
pub mod api_doc;
pub mod env;
pub mod middleware;
pub mod routes;

#[derive(Debug)]
pub struct Tracker {
    env: Env,

    pub users: RwLock<arcadia_shared::tracker::models::user::Map>,
    pub passkey2id: RwLock<arcadia_shared::tracker::models::passkey_2_id::Map>,
    pub torrents: RwLock<arcadia_shared::tracker::models::torrent::Map>,
}

impl Deref for Tracker {
    type Target = Env;

    fn deref(&self) -> &Self::Target {
        &self.env
    }
}

impl Tracker {
    pub async fn new(env: Env) -> Self {
        log::info!("[Setup] Getting users...");
        std::io::stdout().flush().unwrap();
        let users = arcadia_shared::tracker::models::user::Map::from_backend().await;
        log::info!("[Setup] Got {:?} users", users.len());

        log::info!("[Setup] Getting passkey2id...");
        std::io::stdout().flush().unwrap();
        let passkey2id = arcadia_shared::tracker::models::passkey_2_id::Map::from_backend().await;
        log::info!("[Setup] Got {:?} passkey2ids", passkey2id.len());

        log::info!("[Setup] Getting torrents...");
        std::io::stdout().flush().unwrap();
        let torrents = arcadia_shared::tracker::models::torrent::Map::from_backend().await;
        log::info!("[Setup] Got {:?} torrents", torrents.len());

        Self {
            env,
            users: RwLock::new(users),
            passkey2id: RwLock::new(passkey2id),
            torrents: RwLock::new(torrents),
        }
    }
}
