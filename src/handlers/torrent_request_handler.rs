use crate::{
    Arcadia, Result,
    models::{torrent_request::UserCreatedTorrentRequest, user::User},
    repositories::torrent_request_repository::create_torrent_request,
};
use actix_web::{HttpResponse, web};

pub async fn add_torrent_request(
    torrent_request: web::Json<UserCreatedTorrentRequest>,
    arc: web::Data<Arcadia>,
    current_user: User,
) -> Result<HttpResponse> {
    let torrent_request =
        create_torrent_request(&arc.pool, &torrent_request, &current_user).await?;

    Ok(HttpResponse::Created().json(torrent_request))
}
