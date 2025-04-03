use crate::{
    Arcadia,
    models::{title_group_comment::UserCreatedTitleGroupComment, user::User},
    repositories::title_group_comment_repository::create_title_group_comment,
};
use actix_web::{HttpResponse, web};

pub async fn add_title_group_comment(
    comment: web::Json<UserCreatedTitleGroupComment>,
    arc: web::Data<Arcadia>,
    current_user: User,
) -> HttpResponse {
    match create_title_group_comment(&arc.pool, &comment, &current_user).await {
        Ok(created_comment) => HttpResponse::Created().json(serde_json::json!(created_comment)),
        Err(err) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": err.to_string()
        })),
    }
}
