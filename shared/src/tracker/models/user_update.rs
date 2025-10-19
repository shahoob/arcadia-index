use serde::{Deserialize, Serialize};

use crate::tracker::models::Mergeable;

// Fields must be in same order as database primary key
#[derive(Debug, Eq, Hash, PartialEq, PartialOrd, Ord, bincode::Encode, bincode::Decode)]
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
