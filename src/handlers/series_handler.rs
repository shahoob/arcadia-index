use std::collections::HashMap;

use crate::{
    models::{series::UserCreatedSeries, user::User},
    repositories::series_repository::{create_series, find_series},
};
use actix_web::{HttpResponse, web};
use sqlx::PgPool;

pub async fn add_series(
    serie: web::Json<UserCreatedSeries>,
    pool: web::Data<PgPool>,
    current_user: User,
) -> HttpResponse {
    match create_series(&pool, &serie, &current_user).await {
        Ok(created_serie) => HttpResponse::Created().json(serde_json::json!(created_serie)),
        Err(err) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": err.to_string()
        })),
    }
}

pub async fn get_series(
    pool: web::Data<PgPool>,
    query: web::Query<HashMap<String, String>>,
) -> HttpResponse {
    let series_id = query.get("id").expect("id not found in query");
    match find_series(&pool, &series_id.parse::<i64>().unwrap()).await {
        Ok(title_group) => HttpResponse::Ok().json(title_group),
        Err(err) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": err.to_string()
        })),
    }
}
