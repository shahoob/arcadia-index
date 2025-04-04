use crate::{
    Arcadia, Result,
    models::user::User,
    repositories::subscriptions_repository::{create_subscription, delete_subscription},
};
use actix_web::{HttpResponse, web};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AddSubscriptionQuery {
    item_id: i64,
    item: String,
}

pub async fn add_subscription(
    query: web::Query<AddSubscriptionQuery>,
    arc: web::Data<Arcadia>,
    current_user: User,
) -> Result<HttpResponse> {
    create_subscription(&arc.pool, &query.item_id, &query.item, &current_user).await?;

    Ok(HttpResponse::Created().json(serde_json::json!({"result": "success"})))
}

pub type RemoveSubscriptionQuery = AddSubscriptionQuery;

pub async fn remove_subscription(
    query: web::Query<RemoveSubscriptionQuery>,
    arc: web::Data<Arcadia>,
    current_user: User,
) -> Result<HttpResponse> {
    delete_subscription(&arc.pool, &query.item_id, &query.item, &current_user).await?;

    Ok(HttpResponse::Ok().json(serde_json::json!({"result": "success"})))
}
