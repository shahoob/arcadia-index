use chrono::{DateTime, Utc};
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};

use crate::tracker::models::{peer_id::PeerId, Flushable, Mergeable, Queue};

// Fields must be in same order as database primary key
#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    Eq,
    Hash,
    PartialEq,
    PartialOrd,
    Ord,
    bincode::Encode,
    bincode::Decode,
)]
pub struct Index {
    pub user_id: u32,
    pub torrent_id: u32,
    pub peer_id: PeerId,
}

#[derive(Debug, Clone, Serialize, Deserialize, bincode::Encode, bincode::Decode)]
pub struct PeerUpdate {
    pub ip: std::net::IpAddr,
    pub port: u16,
    pub agent: String,
    pub uploaded: u64,
    pub downloaded: u64,
    pub is_active: bool,
    pub is_seeder: bool,
    pub left: u64,
    #[bincode(with_serde)]
    pub created_at: DateTime<Utc>,
    #[bincode(with_serde)]
    pub updated_at: DateTime<Utc>,
}

impl Mergeable for PeerUpdate {
    fn merge(&mut self, new: &Self) {
        if new.updated_at > self.updated_at {
            self.ip = new.ip;
            self.port = new.port;
            self.agent = new.agent.clone();
            self.uploaded = new.uploaded;
            self.downloaded = new.downloaded;
            self.is_active = new.is_active;
            self.is_seeder = new.is_seeder;
            self.left = new.left;
            self.updated_at = new.updated_at;
        }

        self.created_at = std::cmp::min(self.created_at, new.created_at);
    }
}

impl Flushable<PeerUpdate> for Mutex<Queue<Index, PeerUpdate>> {
    async fn flush_to_backend(&self) {
        let amount_of_updates = self.lock().records.len();
        let updates = self
            .lock()
            .records
            .drain(0..amount_of_updates)
            .collect::<Vec<(Index, PeerUpdate)>>();
        if updates.is_empty() {
            return;
        }
        let base_url =
            std::env::var("ARCADIA_API_BASE_URL").expect("env var ARCADIA_API_BASE_URL not set");
        let url = format!("{}/api/tracker/peer-updates", base_url);

        let client = reqwest::Client::new();
        let api_key = std::env::var("API_KEY").expect("env var API_KEY not set");

        let config = bincode::config::standard();
        let bytes = bincode::encode_to_vec(updates, config).expect("error encoding to bincode");

        let response = client
            .post(url)
            .header("api_key", api_key)
            .header("Content-Type", "application/octet-stream")
            .body(bytes)
            .send()
            .await
            .expect("failed to send peer updates to backend");

        if !response.status().is_success() {
            // TODO: reinsert the updates that failed and retry
            panic!("Backend returned error: {}", response.text().await.unwrap());
        } else {
            log::info!("Inserted {amount_of_updates} peer updates");
        }
    }
}
