use crate::tracker::models::{Flushable, Mergeable, Queue};
use chrono::{DateTime, Local};
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};

// TODO: use this to populate the torrent activites table
// or something else if we find a better solution
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
    pub user_id: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, bincode::Encode, bincode::Decode)]
pub struct TorrentActivityUpdate {
    #[bincode(with_serde)]
    pub snatched_at: Option<DateTime<Local>>,
    #[bincode(with_serde)]
    pub last_seen_seeding_at: Option<DateTime<Local>>,
    pub uploaded_delta: u64,
    pub real_uploaded_delta: u64,
    pub downloaded_delta: u64,
    pub real_downloaded_delta: u64,
    pub seed_time_delta: u64,
}

impl Mergeable for TorrentActivityUpdate {
    fn merge(&mut self, new: &Self) {
        if new.snatched_at.is_some() {
            self.snatched_at = new.snatched_at;
        }
        self.last_seen_seeding_at = new.last_seen_seeding_at;
        self.uploaded_delta = self.uploaded_delta.saturating_add(new.uploaded_delta);
        self.real_uploaded_delta = self
            .real_uploaded_delta
            .saturating_add(new.real_uploaded_delta);
        self.downloaded_delta = self.downloaded_delta.saturating_add(new.downloaded_delta);
        self.real_downloaded_delta = self
            .real_downloaded_delta
            .saturating_add(new.real_downloaded_delta);
        // should be calculated during announce by comparing with the previous announce datetime
        self.seed_time_delta = self.seed_time_delta.saturating_add(new.seed_time_delta);
    }
}

impl Flushable<TorrentActivityUpdate> for Mutex<Queue<Index, TorrentActivityUpdate>> {
    async fn flush_to_backend(&self) {
        let amount_of_updates = self.lock().records.len();
        let updates = self
            .lock()
            .records
            .drain(0..amount_of_updates)
            .collect::<Vec<(Index, TorrentActivityUpdate)>>();
        if updates.is_empty() {
            return;
        }
        let base_url =
            std::env::var("ARCADIA_API_BASE_URL").expect("env var ARCADIA_API_BASE_URL not set");
        let url = format!("{}/api/tracker/torrent-activity-updates", base_url);

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
            .expect("failed to send torrent activity updates to backend");

        if !response.status().is_success() {
            // TODO: reinsert the updates that failed and retry
            panic!("Backend returned error: {}", response.text().await.unwrap());
        } else {
            log::info!("Inserted {amount_of_updates} torrent activity updates");
        }
    }
}
