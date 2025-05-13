use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct TitleGroupSubscription {
    pub id: i64,
    pub subscribed_at: DateTime<Local>,
    pub subscriber_id: i64,
    pub title_group_id: i64,
}
