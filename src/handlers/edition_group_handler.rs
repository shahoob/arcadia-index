use crate::{
    Arcadia,
    models::{edition_group::UserCreatedEditionGroup, user::User},
    repositories::edition_group_repository::create_edition_group,
};
use actix_web::{HttpResponse, web};

pub async fn add_edition_group(
    form: web::Json<UserCreatedEditionGroup>,
    arc: web::Data<Arcadia>,
    current_user: User,
) -> HttpResponse {
    match create_edition_group(&arc.pool, &form, &current_user).await {
        Ok(edition_group) => HttpResponse::Created().json(serde_json::json!(edition_group)),
        Err(err) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": err.to_string()
        })),
    }
}
