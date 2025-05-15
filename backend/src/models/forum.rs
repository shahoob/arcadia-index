use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, Serialize, FromRow, ToSchema)]
pub struct ForumCategory {
    pub id: i32,
    pub name: String,
    #[schema(value_type = String, format = DateTime)]
    pub created_at: DateTime<Local>,
    pub created_by_id: i64,
}

#[derive(Debug, Deserialize, Serialize, FromRow, ToSchema)]
pub struct ForumSubCategory {
    pub id: i32,
    pub forum_category_id: i32,
    pub name: String,
    #[schema(value_type = String, format = DateTime)]
    pub created_at: DateTime<Local>,
    pub created_by_id: i64,
    pub thread_amount: i64,
    pub post_amount: i64,
}

#[derive(Debug, Deserialize, Serialize, FromRow, ToSchema)]
pub struct ForumThread {
    pub id: i64,
    pub forum_sub_category_id: i32,
    pub name: String,
    #[schema(value_type = String, format = DateTime)]
    pub created_at: DateTime<Local>,
    pub created_by_id: i64,
    pub post_amount: i64,
}

#[derive(Debug, Deserialize, Serialize, FromRow, ToSchema)]
pub struct ForumPost {
    pub id: i64,
    pub forum_thread_id: i64,
    #[schema(value_type = String, format = DateTime)]
    pub created_at: DateTime<Local>,
    #[schema(value_type = String, format = DateTime)]
    pub updated_at: DateTime<Local>,
    pub created_by_id: i64,
    pub content: String,
}

#[derive(Debug, Deserialize, Serialize, FromRow, ToSchema)]
pub struct UserCreatedForumPost {
    pub content: String,
    pub forum_thread_id: i64,
}
