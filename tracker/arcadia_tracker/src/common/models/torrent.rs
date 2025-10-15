use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};

use arcadia_shared::tracker::models::torrent::{InfoHash, Torrent};
use bincode::config;
use reqwest::Client;

#[derive(Debug)]
pub struct Map(HashMap<InfoHash, Torrent>);

impl Map {
    pub async fn from_backend() -> Self {
        let base_url =
            std::env::var("ARCADIA_API_BASE_URL").expect("env var ARCADIA_API_BASE_URL not set");
        let url = format!("{}/api/tracker/torrents", base_url);

        let client = Client::new();
        let api_key = std::env::var("API_KEY").expect("env var API_KEY not set");
        let resp = client
            .get(url)
            .header("api_key", api_key)
            .send()
            .await
            .expect("failed to fetch torrents");
        let bytes = resp
            .bytes()
            .await
            .expect("failed to read torrents response body");

        let config = config::standard();
        let (torrents, _): (Vec<Torrent>, usize) =
            bincode::decode_from_slice(&bytes[..], config).unwrap();
        let mut map = HashMap::new();
        for torrent in torrents {
            map.insert(torrent.info_hash, torrent);
        }
        Self(map)
    }
}

impl Deref for Map {
    type Target = HashMap<InfoHash, Torrent>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Map {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
