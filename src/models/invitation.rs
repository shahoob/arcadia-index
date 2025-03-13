use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Invitation {
    pub created_at: NaiveDateTime,
    pub expires_at: NaiveDateTime,
    pub message: String,
    pub sender_id: i32,
    pub receiver_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SentInvitation {
    pub message: String,
    pub receiver_username: String,
}
