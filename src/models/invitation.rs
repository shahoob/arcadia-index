use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow, Default)]
pub struct Invitation {
    pub id: i64,
    pub created_at: NaiveDateTime,
    pub expires_at: NaiveDateTime,
    pub message: String,
    pub invitation_key: String,
    pub sender_id: i64,
    pub receiver_email: String,
    pub receiver_id: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SentInvitation {
    pub message: String,
    pub receiver_email: String,
}
