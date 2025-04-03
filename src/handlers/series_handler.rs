use crate::{
    models::{series::UserCreatedSeries, user::User},
    repositories::series_repository::{create_series, find_series},
};
use actix_web::{HttpResponse, web};
use serde::Deserialize;
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

#[derive(Debug, Deserialize)]
pub struct GetSeriesQuery {
    id: i64,
}

pub async fn get_series(
    pool: web::Data<PgPool>,
    query: web::Query<GetSeriesQuery>,
) -> HttpResponse {
    match find_series(&pool, &query.id).await {
        Ok(title_group) => HttpResponse::Ok().json(title_group),
        Err(err) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": err.to_string()
        })),
    }
}
