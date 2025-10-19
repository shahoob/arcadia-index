use bincode::config;
use indexmap::IndexMap;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use sqlx::{Database, Decode};
use std::ops::{Deref, DerefMut};

use crate::tracker::models::peer;

#[derive(
    Clone,
    Copy,
    Serialize,
    Deserialize,
    Debug,
    Eq,
    Hash,
    PartialEq,
    bincode::Encode,
    bincode::Decode,
)]
pub struct InfoHash(pub [u8; 20]);

#[derive(Debug, Clone, Serialize, Deserialize, bincode::Encode, bincode::Decode, PartialEq)]
pub struct Torrent {
    pub upload_factor: i16,
    pub download_factor: i16,
    pub seeders: u32,
    pub leechers: u32,
    pub times_completed: u32,
    pub is_deleted: bool,
    pub peers: peer::Map,
}

#[derive(Debug, bincode::Encode, bincode::Decode)]
pub struct Map(#[bincode(with_serde)] pub IndexMap<u32, Torrent>);

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
        let (map, _): (Map, usize) = bincode::decode_from_slice(&bytes[..], config).unwrap();
        map
    }
}

impl Deref for Map {
    type Target = IndexMap<u32, Torrent>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Map {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'r, DB: Database> Decode<'r, DB> for InfoHash
where
    &'r [u8]: Decode<'r, DB>,
{
    /// Decodes the database's string representation of the 40 character long
    /// infohash in hex into a byte slice
    fn decode(
        value: <DB as Database>::ValueRef<'r>,
    ) -> Result<InfoHash, Box<dyn std::error::Error + 'static + Send + Sync>> {
        let value = <&[u8] as Decode<DB>>::decode(value)?;

        if value.len() != 20 {
            let error: Box<dyn std::error::Error + Send + Sync> =
                Box::new(crate::error::DecodeError::InfoHash);

            return Err(error);
        }

        Ok(InfoHash(<[u8; 20]>::try_from(&value[0..20])?))
    }
}
