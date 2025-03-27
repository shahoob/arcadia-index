use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct TitleGroupSubscription {
    pub id: i64,
    pub subscribed_at: NaiveDateTime,
    pub subscriber_id: i32,
    pub title_group_id: i64,
}
