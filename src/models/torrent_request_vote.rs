use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct TorrentRequestVote {
    pub id: i64,
    pub torrent_request_id: i64,
    pub created_at: NaiveDateTime,
    pub created_by_id: i32,
    pub bounty_upload: i64,
    pub bounty_bonus_points: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserCreatedTorrentRequestVote {
    pub torrent_request_id: i64,
    pub bounty_upload: i64,
    pub bounty_bonus_points: i64,
}
