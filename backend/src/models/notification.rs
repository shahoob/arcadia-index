use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Deserialize, Serialize)]
pub enum NotificationItem {
    TitleGroup,
    Artist,
    Collage,
    Other,
}

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct Notification {
    pub id: i64,
    pub created_at: DateTime<Local>,
    pub receiver_id: i64,
    pub title: String,
    pub message: String,
    pub item: NotificationItem,
    pub read_status: bool,
    pub item_id: Option<i64>,
}
