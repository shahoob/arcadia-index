use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Series {
    pub id: i64,
    pub name: String,
    pub description: String,
    #[schema(value_type = String, format = DateTime)]
    pub created_at: NaiveDateTime,
    #[schema(value_type = String, format = DateTime)]
    pub updated_at: NaiveDateTime,
    pub created_by_id: i64,
    pub covers: Vec<String>,
    pub banners: Option<Vec<String>>,
    pub tags: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserCreatedSeries {
    pub name: String,
    pub description: String,
    pub covers: Vec<String>,
    pub banners: Vec<String>,
    pub tags: Vec<String>,
}
