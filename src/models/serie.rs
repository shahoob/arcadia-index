use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Serie {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub created_by_id: i32,
    pub covers: Vec<String>,
    pub fannarts: Vec<String>,
    pub banners: Vec<String>,
    pub tags: Vec<String>,
}
