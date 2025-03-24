use crate::{
    models::{title_group_comment::UserCreatedTitleGroupComment, user::User},
    repositories::title_group_comment_repository::create_title_group_comment,
};
use actix_web::{HttpResponse, web};
use sqlx::PgPool;

pub async fn add_title_group_comment(
    comment: web::Json<UserCreatedTitleGroupComment>,
    pool: web::Data<PgPool>,
    current_user: User,
) -> HttpResponse {
    match create_title_group_comment(&pool, &comment, &current_user).await {
        Ok(created_comment) => HttpResponse::Created().json(serde_json::json!(created_comment)),
        Err(err) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": err.to_string()
        })),
    }
}
