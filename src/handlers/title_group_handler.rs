use actix_web::{HttpResponse, web};
use sqlx::PgPool;

use crate::{
    models::{title_group::UserCreatedTitleGroup, user::User},
    repositories::title_group_repository::create_title_group,
};

pub async fn add_title_group(
    form: web::Json<UserCreatedTitleGroup>,
    pool: web::Data<PgPool>,
    current_user: User,
) -> HttpResponse {
    match create_title_group(&pool, &form, &current_user).await {
        Ok(title_group) => HttpResponse::Created().json(serde_json::json!(title_group)),
        Err(err) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": err.to_string()
        })),
    }
}
