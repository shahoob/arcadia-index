use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use utoipa::ToSchema;

use super::torrent::{AudioCodec, Features, VideoCodec};

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct TorrentRequest {
    pub id: i64,
    pub title_group_id: i64,
    #[schema(value_type = String, format = DateTime)]
    pub created_at: NaiveDateTime,
    #[schema(value_type = String, format = DateTime)]
    pub updated_at: NaiveDateTime,
    pub created_by_id: i64,
    pub edition_name: Option<String>,
    pub release_group: Option<String>,
    pub description: Option<String>,
    pub language: Option<String>,
    pub container: String,
    pub bounty_upload: i64,
    pub bounty_bonus_points: i64,
    // ---- audio
    pub audio_codec: Option<AudioCodec>,
    pub audio_channels: Option<String>,
    // ---- audio
    // ---- video
    pub video_codec: Option<VideoCodec>,
    pub features: Option<Vec<Features>>,
    pub subtitle_languages: Option<Vec<String>>,
    pub video_resolution: Option<String>, // ---- video
}
#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct UserCreatedTorrentRequest {
    pub title_group_id: i64,
    pub edition_name: Option<String>,
    pub release_group: Option<String>,
    pub description: Option<String>,
    pub language: Option<String>,
    pub container: String,
    pub bounty_upload: i64,
    pub bounty_bonus_points: i64,
    // ---- audio
    pub audio_codec: Option<AudioCodec>,
    pub audio_channels: Option<String>,
    // ---- audio
    // ---- video
    pub video_codec: Option<VideoCodec>,
    pub features: Option<Vec<Features>>,
    pub subtitle_languages: Option<Vec<String>>,
    pub video_resolution: Option<String>, // ---- video
}
