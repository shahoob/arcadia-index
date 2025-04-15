use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct TorrentRequestVote {
    pub id: i64,
    pub torrent_request_id: i64,
    #[schema(value_type = String, format = DateTime)]
    pub created_at: NaiveDateTime,
    pub created_by_id: i64,
    pub bounty_upload: i64,
    pub bounty_bonus_points: i64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserCreatedTorrentRequestVote {
    pub torrent_request_id: i64,
    pub bounty_upload: i64,
    pub bounty_bonus_points: i64,
}
