use crate::{
    Arcadia, Result, models::user_application::UserCreatedUserApplication,
    repositories::user_application_repository::create_user_application,
};
use actix_web::{HttpResponse, web};
use serde_json::json;

#[utoipa::path(
    post,
    path = "/api/user-application",
    responses(
        (status = 200, description = "Successfully sent the application"),
    )
)]
pub async fn add_user_application(
    user_application: web::Json<UserCreatedUserApplication>,
    arc: web::Data<Arcadia>,
) -> Result<HttpResponse> {
    let _ = create_user_application(&arc.pool, &user_application).await?;

    Ok(HttpResponse::Created().json(json!({"status": "success"})))
}
