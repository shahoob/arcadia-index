use bincode::config;
use reqwest::Client;
use serde::Serialize;
use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};

pub use arcadia_shared::tracker::models::user::{Passkey, User};

#[derive(Debug, Serialize)]
pub struct Map(HashMap<Passkey, User>);

impl Map {
    pub async fn from_backend() -> Self {
        let base_url =
            std::env::var("ARCADIA_API_BASE_URL").expect("env var ARCADIA_API_BASE_URL not set");
        let url = format!("{}/api/tracker/users", base_url);

        let client = Client::new();
        let api_key = std::env::var("API_KEY").expect("env var API_KEY not set");
        let resp = client
            .get(url)
            .header("api_key", api_key)
            .send()
            .await
            .expect("failed to fetch users");
        let bytes = resp
            .bytes()
            .await
            .expect("failed to read users response body");

        let config = config::standard();
        let (users, _): (Vec<User>, usize) =
            bincode::decode_from_slice(&bytes[..], config).unwrap();
        let mut map = HashMap::new();
        for user in users {
            map.insert(user.passkey, user);
        }
        Self(map)
    }
}

impl Deref for Map {
    type Target = HashMap<Passkey, User>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Map {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
