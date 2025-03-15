use actix_multipart::form::MultipartForm;
use actix_web::{HttpResponse, web};
use sqlx::PgPool;

use crate::{
    models::{torrent::UploadedTorrent, user::User},
    repositories::torrent_repository::create_torrent,
};

pub async fn upload_torrent(
    form: MultipartForm<UploadedTorrent>,
    pool: web::Data<PgPool>,
    current_user: User,
) -> HttpResponse {
    // TODO : check if user can upload

    match create_torrent(&pool, &form, &current_user).await {
        Ok(created_torrent) => HttpResponse::Created().json(serde_json::json!(created_torrent)),
        Err(err) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": err.to_string()
        })),
    }
}
