use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct TitleGroupComment {
    pub id: i64,
    pub content: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub created_by_id: i32,
    pub title_group_id: i32,
    pub refers_to_torrent_id: Option<i32>,
    pub answers_to_comment_id: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserCreatedTitleGroupComment {
    pub content: String,
    pub title_group_id: i32,
    pub refers_to_torrent_id: Option<i32>,
    pub answers_to_comment_id: Option<i64>,
}
