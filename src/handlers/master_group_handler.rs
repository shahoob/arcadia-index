use crate::{
    Arcadia,
    models::{master_group::UserCreatedMasterGroup, user::User},
    repositories::master_group_repository::create_master_group,
};
use actix_web::{HttpResponse, web};

pub async fn add_master_group(
    form: web::Json<UserCreatedMasterGroup>,
    arc: web::Data<Arcadia>,
    current_user: User,
) -> HttpResponse {
    match create_master_group(&arc.pool, &form, &current_user).await {
        Ok(master_group) => HttpResponse::Created().json(serde_json::json!(master_group)),
        Err(err) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": err.to_string()
        })),
    }
}
