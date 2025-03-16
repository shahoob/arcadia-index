use actix_web::{HttpResponse, web};
use sqlx::PgPool;

use crate::{
    models::{artist::UserCreatedArtist, torrent::UploadedTorrent, user::User},
    repositories::{artist_repository::create_artist, torrent_repository::create_torrent},
};

pub async fn add_artist(
    artist: web::Json<UserCreatedArtist>,
    pool: web::Data<PgPool>,
    current_user: User,
) -> HttpResponse {
    match create_artist(&pool, &artist, &current_user).await {
        Ok(created_artist) => HttpResponse::Created().json(serde_json::json!(created_artist)),
        Err(err) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": err.to_string()
        })),
    }
}
