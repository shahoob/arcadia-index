use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Series {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub created_by_id: i32,
    pub covers: Vec<String>,
    pub banners: Option<Vec<String>>,
    pub tags: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserCreatedSeries {
    pub name: String,
    pub description: String,
    pub covers: Vec<String>,
    pub banners: Option<Vec<String>>,
    pub tags: Vec<String>,
}
