use crate::{
    Arcadia,
    models::{torrent_request::UserCreatedTorrentRequest, user::User},
    repositories::torrent_request_repository::create_torrent_request,
};
use actix_web::{HttpResponse, web};

pub async fn add_torrent_request(
    torrent_request: web::Json<UserCreatedTorrentRequest>,
    arc: web::Data<Arcadia>,
    current_user: User,
) -> HttpResponse {
    match create_torrent_request(&arc.pool, &torrent_request, &current_user).await {
        Ok(created_torrent_request) => {
            HttpResponse::Created().json(serde_json::json!(created_torrent_request))
        }
        Err(err) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": err.to_string()
        })),
    }
}
