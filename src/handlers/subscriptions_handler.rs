use std::collections::HashMap;

use crate::{
    models::user::User,
    repositories::subscriptions_repository::{create_subscription, delete_subscription},
};
use actix_web::{HttpResponse, web};
use sqlx::PgPool;

pub async fn add_subscription(
    query: web::Query<HashMap<String, String>>,
    pool: web::Data<PgPool>,
    current_user: User,
) -> HttpResponse {
    let item_id = query.get("item_id").expect("id not found in query");
    let item = query.get("item").expect("item not found in query");
    match create_subscription(
        &pool,
        &item_id.parse::<i64>().unwrap(),
        &item,
        &current_user,
    )
    .await
    {
        Ok(_) => HttpResponse::Created().json(serde_json::json!({"result": "success"})),
        Err(err) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": err.to_string()
        })),
    }
}

pub async fn remove_subscription(
    query: web::Query<HashMap<String, String>>,
    pool: web::Data<PgPool>,
    current_user: User,
) -> HttpResponse {
    let item_id = query.get("item_id").expect("id not found in query");
    let item = query.get("item").expect("item not found in query");
    match delete_subscription(
        &pool,
        &item_id.parse::<i64>().unwrap(),
        &item,
        &current_user,
    )
    .await
    {
        Ok(_) => HttpResponse::Created().json(serde_json::json!({"result": "success"})),
        Err(err) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": err.to_string()
        })),
    }
}
