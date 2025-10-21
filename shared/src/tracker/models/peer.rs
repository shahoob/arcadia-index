use std::fmt::Display;
use std::ops::{Deref, DerefMut};

use chrono::serde::ts_seconds;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize, Serializer};
use sqlx::types::chrono::{DateTime, Utc};

use crate::tracker::models::peer_id::PeerId;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, bincode::Encode, bincode::Decode)]
pub struct Map(#[bincode(with_serde)] IndexMap<Index, Peer>);

#[derive(
    Debug, Clone, Copy, Deserialize, PartialEq, Eq, Hash, bincode::Encode, bincode::Decode,
)]
pub struct Index {
    pub user_id: u32,
    pub peer_id: PeerId,
}

#[derive(
    Clone, Copy, Debug, Serialize, Deserialize, PartialEq, bincode::Encode, bincode::Decode,
)]
pub struct Peer {
    pub ip_address: std::net::IpAddr,
    pub port: u16,
    pub is_seeder: bool,
    pub is_active: bool,
    // pub is_visible: bool,
    pub has_sent_completed: bool,
    #[serde(with = "ts_seconds")]
    #[bincode(with_serde)]
    pub updated_at: DateTime<Utc>,
    pub uploaded: u64,
    pub downloaded: u64,
}

impl Peer {
    /// Determines if the peer should be included in the peer list
    #[inline(always)]
    pub fn is_included_in_peer_list(&self) -> bool {
        self.is_active //&& self.is_visible
    }

    /// Determines if the peer should be included in the list of seeds
    #[inline(always)]
    pub fn is_included_in_seed_list(&self) -> bool {
        self.is_seeder && self.is_included_in_peer_list()
    }

    /// Determines if the peer should be included in the list of leeches
    #[inline(always)]
    pub fn is_included_in_leech_list(&self) -> bool {
        !self.is_seeder && self.is_included_in_peer_list()
    }
}

impl Map {
    pub fn new() -> Map {
        Map(IndexMap::new())
    }
}

impl Deref for Map {
    type Target = IndexMap<Index, Peer>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Map {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Default for Map {
    fn default() -> Self {
        Map::new()
    }
}

impl Display for Index {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", self.user_id, self.peer_id)
    }
}

impl Serialize for Index {
    fn serialize<S>(&self, serializer: S) -> std::prelude::v1::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
