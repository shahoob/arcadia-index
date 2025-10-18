use anyhow::bail;
use bincode::config;
use indexmap::IndexMap;
use reqwest::Client;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use sqlx::{Database, Decode};
use std::{
    fmt::Display,
    ops::{Deref, DerefMut},
    str::FromStr,
};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, bincode::Encode, bincode::Decode)]
pub struct Passkey(pub [u8; 32]);

#[derive(Debug, Clone, Deserialize, Serialize, bincode::Encode, bincode::Decode, PartialEq)]
pub struct User {
    pub num_seeding: u32,
    pub num_leeching: u32,
}

#[derive(Debug, Serialize, bincode::Encode, bincode::Decode)]
pub struct Map(#[bincode(with_serde)] pub IndexMap<u32, User>);

impl FromStr for Passkey {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut bytes = s.bytes();

        if bytes.len() != 32 {
            bail!("Invalid passkey length.");
        }

        let array = [(); 32].map(|_| bytes.next().unwrap());

        Ok(Passkey(array))
    }
}

impl<'r, DB: Database> Decode<'r, DB> for Passkey
where
    &'r str: Decode<'r, DB>,
{
    fn decode(
        value: <DB as Database>::ValueRef<'r>,
    ) -> Result<Passkey, Box<dyn std::error::Error + 'static + Send + Sync>> {
        let value = <&str as Decode<DB>>::decode(value)?;
        let mut bytes = value.bytes();

        let array = [(); 32].map(|_| bytes.next().expect("Invalid passkey length."));

        Ok(Passkey(array))
    }
}

impl Display for Passkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&String::from_utf8_lossy(&self.0))
    }
}

impl Serialize for Passkey {
    fn serialize<S>(&self, serializer: S) -> std::prelude::v1::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for Passkey {
    fn deserialize<D>(deserializer: D) -> std::prelude::v1::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse().map_err(serde::de::Error::custom)
    }
}

impl Deref for Map {
    type Target = IndexMap<u32, User>;

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
        let (map, _): (Map, usize) = bincode::decode_from_slice(&bytes[..], config).unwrap();

        map
    }
}
