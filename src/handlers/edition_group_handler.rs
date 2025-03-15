use actix_web::{HttpResponse, web};
use sqlx::PgPool;

use crate::{
    models::{edition_group::UserCreatedEditionGroup, user::User},
    repositories::edition_group_repository::create_edition_group,
};

pub async fn add_edition_group(
    form: web::Json<UserCreatedEditionGroup>,
    pool: web::Data<PgPool>,
    current_user: User,
) -> HttpResponse {
    match create_edition_group(&pool, &form, &current_user).await {
        Ok(edition_group) => HttpResponse::Created().json(serde_json::json!(edition_group)),
        Err(err) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": err.to_string()
        })),
    }
}
