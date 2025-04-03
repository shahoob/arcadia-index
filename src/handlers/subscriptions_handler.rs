use crate::{
    models::user::User,
    repositories::subscriptions_repository::{create_subscription, delete_subscription},
};
use actix_web::{HttpResponse, web};
use serde::Deserialize;
use sqlx::PgPool;

#[derive(Debug, Deserialize)]
pub struct AddSubscriptionQuery {
    item_id: i64,
    item: String,
}

pub async fn add_subscription(
    query: web::Query<AddSubscriptionQuery>,
    pool: web::Data<PgPool>,
    current_user: User,
) -> HttpResponse {
    match create_subscription(&pool, &query.item_id, &query.item, &current_user).await {
        Ok(_) => HttpResponse::Created().json(serde_json::json!({"result": "success"})),
        Err(err) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": err.to_string()
        })),
    }
}

pub type RemoveSubscriptionQuery = AddSubscriptionQuery;

pub async fn remove_subscription(
    query: web::Query<RemoveSubscriptionQuery>,
    pool: web::Data<PgPool>,
    current_user: User,
) -> HttpResponse {
    match delete_subscription(&pool, &query.item_id, &query.item, &current_user).await {
        Ok(_) => HttpResponse::Created().json(serde_json::json!({"result": "success"})),
        Err(err) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": err.to_string()
        })),
    }
}
