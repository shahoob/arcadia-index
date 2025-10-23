use crate::tracker::models::{Flushable, Mergeable, Queue};
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};

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
    pub torrent_id: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, bincode::Encode, bincode::Decode)]
pub struct TorrentUpdate {
    pub seeder_delta: i32,
    pub leecher_delta: i32,
    pub times_completed_delta: u32,
}

impl Mergeable for TorrentUpdate {
    fn merge(&mut self, new: &Self) {
        self.seeder_delta = self.seeder_delta.saturating_add(new.seeder_delta);
        self.leecher_delta = self.leecher_delta.saturating_add(new.leecher_delta);
        self.times_completed_delta = self
            .times_completed_delta
            .saturating_add(new.times_completed_delta);
    }
}

impl Flushable<TorrentUpdate> for Mutex<Queue<Index, TorrentUpdate>> {
    async fn flush_to_backend(&self) {
        let amount_of_updates = self.lock().records.len();
        let updates = self
            .lock()
            .records
            .drain(0..amount_of_updates)
            .collect::<Vec<(Index, TorrentUpdate)>>();
        if updates.is_empty() {
            return;
        }
        let base_url =
            std::env::var("ARCADIA_API_BASE_URL").expect("env var ARCADIA_API_BASE_URL not set");
        let url = format!("{}/api/tracker/torrent-updates", base_url);

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
            .expect("failed to send user updates to backend");

        if !response.status().is_success() {
            // TODO: reinsert the updates that failed and retry
            panic!("Backend returned error: {}", response.text().await.unwrap());
        } else {
            log::info!("Inserted {amount_of_updates} torrent updates");
        }
    }
}
