use crate::tracker::models::torrent::InfoHash;
use bincode::config;
use reqwest::Client;
use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};

#[derive(Debug, bincode::Encode, bincode::Decode)]
pub struct Map(HashMap<InfoHash, u32>);

impl Deref for Map {
    type Target = HashMap<InfoHash, u32>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Map {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Map {
    pub async fn from_backend() -> Self {
        let base_url =
            std::env::var("ARCADIA_API_BASE_URL").expect("env var ARCADIA_API_BASE_URL not set");
        let url = format!("{}/api/tracker/infohashes-2-ids", base_url);

        let client = Client::new();
        let api_key = std::env::var("API_KEY").expect("env var API_KEY not set");
        let resp = client
            .get(url)
            .header("api_key", api_key)
            .send()
            .await
            .expect("failed to fetch infohashes to ids");
        let bytes = resp
            .bytes()
            .await
            .expect("failed to read infohashes to ids response body");

        let config = config::standard();
        let (map, _): (Map, usize) = bincode::decode_from_slice(&bytes[..], config).unwrap();

        map
    }
}
