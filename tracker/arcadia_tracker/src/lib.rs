use parking_lot::RwLock;

use crate::env::Env;
use std::{io::Write, ops::Deref};

pub mod announce;
pub mod api_doc;
pub mod common;
pub mod env;
pub mod middleware;
pub mod routes;

pub struct Tracker {
    env: Env,

    pub users: RwLock<common::models::user::Map>,
    pub torrents: RwLock<common::models::torrent::Map>,
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
        let users = common::models::user::Map::from_backend().await;
        log::info!("[Setup] Got {:?} users", users.len());

        log::info!("[Setup] Getting torrents...");
        std::io::stdout().flush().unwrap();
        let torrents = common::models::torrent::Map::from_backend().await;
        log::info!("[Setup] Got {:?} torrents", torrents.len());

        Self {
            env,
            users: RwLock::new(users),
            torrents: RwLock::new(torrents),
        }
    }
}
