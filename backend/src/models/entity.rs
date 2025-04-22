use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, Serialize, FromRow, ToSchema)]
pub struct Entity {
    pub id: i64,
    pub name: String,
    #[schema(value_type = String, format = DateTime)]
    pub created_at: NaiveDateTime,
    pub created_by_id: i64,
    pub description: String,
    pub pictures: Option<Vec<String>>,
    pub title_groups_amount: i32,
    pub edition_groups_amount: i32,
    pub torrents_amount: i32,
    pub seeders_amount: i32,
    pub leechers_amount: i32,
    pub snatches_amount: i32,
}
