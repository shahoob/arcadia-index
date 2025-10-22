use parking_lot::Mutex;
use serde::{Deserialize, Serialize};

use crate::tracker::models::{Flushable, Mergeable, Queue};

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
}

#[derive(Debug, Clone, Serialize, Deserialize, bincode::Encode, bincode::Decode)]
pub struct UserUpdate {
    pub uploaded_delta: u64,
    pub downloaded_delta: u64,
}

impl Mergeable for UserUpdate {
    fn merge(&mut self, new: &Self) {
        self.uploaded_delta = self.uploaded_delta.saturating_add(new.uploaded_delta);
        self.downloaded_delta = self.downloaded_delta.saturating_add(new.downloaded_delta);
    }
}

impl Flushable<UserUpdate> for Mutex<Queue<Index, UserUpdate>> {
    async fn flush_to_backend(&self) {
        let amount_of_updates = self.lock().records.len();
        let updates = self
            .lock()
            .records
            .drain(0..amount_of_updates)
            .collect::<Vec<(Index, UserUpdate)>>();
        if updates.is_empty() {
            return;
        }
        let base_url =
            std::env::var("ARCADIA_API_BASE_URL").expect("env var ARCADIA_API_BASE_URL not set");
        let url = format!("{}/api/tracker/user-updates", base_url);

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
            log::info!("Inserted {amount_of_updates} user updates");
        }
    }
}
