use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Collage {
    pub id: i32,
    pub created_at: NaiveDateTime,
    pub created_by_id: i32,
    pub name: String,
    pub cover: String,
    pub description: String,
    pub tags: Vec<String>,
    pub category: String,
}
