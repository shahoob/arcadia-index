use std::collections::HashMap;

use crate::{models::user::User, repositories::user_repository::find_user_by_id};
use actix_web::{HttpResponse, web};
use sqlx::PgPool;

pub async fn get_user(
    pool: web::Data<PgPool>,
    query: web::Query<HashMap<String, String>>,
) -> HttpResponse {
    let user_id = query.get("id").expect("id not found in query");
    match find_user_by_id(&pool, &user_id.parse::<i64>().unwrap()).await {
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
