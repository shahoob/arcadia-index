use crate::{
    Arcadia, Result, repositories::forum_repository::find_first_thread_posts_in_sub_category,
};
use actix_web::{HttpResponse, web};
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, Serialize, FromRow, ToSchema)]
pub struct ForumPostAndThreadName {
    pub id: i64,
    pub forum_thread_id: i64,
    #[schema(value_type = String, format = DateTime)]
    pub created_at: DateTime<Local>,
    #[schema(value_type = String, format = DateTime)]
    pub updated_at: DateTime<Local>,
    pub created_by_id: i64,
    pub content: String,
    pub sticky: bool,
    pub forum_thread_name: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct HomePage {
    recent_announcements: Vec<ForumPostAndThreadName>,
}

#[utoipa::path(
    get,
    path = "/api/home",
    responses(
        (status = 200, description = "", body=HomePage),
    )
)]
pub async fn get_home(arc: web::Data<Arcadia>) -> Result<HttpResponse> {
    let recent_announcements = find_first_thread_posts_in_sub_category(&arc.pool, 1, 5).await?;

    Ok(HttpResponse::Created().json(HomePage {
        recent_announcements,
    }))
}
