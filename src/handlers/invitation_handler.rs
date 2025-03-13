use actix_web::{HttpResponse, web};
use sqlx::PgPool;

use crate::{
    models::invitation::SentInvitation, repositories::invitation_repository::create_invitation,
};

pub async fn send_invitation(
    invitation: web::Json<SentInvitation>,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    match create_invitation(&pool, &invitation).await {
        Ok(user) => HttpResponse::Created().json(serde_json::json!(user)),
        Err(err) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": err.to_string()
        })),
    }
}
