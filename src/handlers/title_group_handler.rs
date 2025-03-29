use std::collections::HashMap;

use actix_web::{HttpResponse, web};
use sqlx::PgPool;

use crate::{
    models::{title_group::UserCreatedTitleGroup, user::User},
    repositories::title_group_repository::{
        create_title_group, find_lite_title_group_info, find_title_group,
    },
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

pub async fn get_title_group(
    pool: web::Data<PgPool>,
    query: web::Query<HashMap<String, String>>,
    current_user: User,
) -> HttpResponse {
    let title_group_id = query.get("id").expect("id not found in query");
    match find_title_group(&pool, title_group_id.parse::<i64>().unwrap(), &current_user).await {
        Ok(title_group) => HttpResponse::Ok().json(title_group),
        Err(err) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": err.to_string()
        })),
    }
}

pub async fn get_lite_title_group_info(
    pool: web::Data<PgPool>,
    query: web::Query<HashMap<String, String>>,
) -> HttpResponse {
    let title_group_id = query.get("id").expect("id not found in query");
    match find_lite_title_group_info(&pool, title_group_id.parse::<i64>().unwrap()).await {
        Ok(title_group) => HttpResponse::Ok().json(title_group),
        Err(err) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": err.to_string()
        })),
    }
}
