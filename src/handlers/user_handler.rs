use crate::{Arcadia, models::user::User, repositories::user_repository::find_user_by_id};
use actix_web::{HttpResponse, web};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GetUserQuery {
    id: i64,
}

pub async fn get_user(arc: web::Data<Arcadia>, query: web::Query<GetUserQuery>) -> HttpResponse {
    match find_user_by_id(&arc.pool, &query.id).await {
        Ok(user) => HttpResponse::Created().json(serde_json::json!(user)),
        Err(err) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": err.to_string()
        })),
    }
}

pub async fn get_me(current_user: User) -> HttpResponse {
    let mut current_user = current_user;
    current_user.password_hash = String::from("");
    HttpResponse::Created().json(serde_json::json!(current_user))
}
