use crate::{
    Arcadia, Result,
    models::{
        torrent_request::{TorrentRequest, TorrentRequestFill, UserCreatedTorrentRequest},
        user::User,
    },
    repositories::torrent_request_repository::{self, create_torrent_request},
};
use actix_web::{HttpResponse, web};
use serde_json::json;

#[utoipa::path(
    post,
    path = "/api/torrent-request",
    responses(
        (status = 200, description = "Successfully created the torrent_request", body=TorrentRequest),
    )
)]
pub async fn add_torrent_request(
    mut torrent_request: web::Json<UserCreatedTorrentRequest>,
    arc: web::Data<Arcadia>,
    current_user: User,
) -> Result<HttpResponse> {
    let torrent_request =
        create_torrent_request(&arc.pool, &mut torrent_request, &current_user).await?;

    Ok(HttpResponse::Created().json(torrent_request))
}

#[utoipa::path(
    post,
    path = "/api/torrent-request/fill",
    responses(
        (status = 200, description = "Successfully filled the torrent request"),
    )
)]
pub async fn fill_torrent_request(
    torrent_request_fill: web::Json<TorrentRequestFill>,
    arc: web::Data<Arcadia>,
    current_user: User,
) -> Result<HttpResponse> {
    torrent_request_repository::fill_torrent_request(
        &arc.pool,
        torrent_request_fill.torrent_id,
        torrent_request_fill.torrent_request_id,
        current_user.id,
    )
    .await?;

    Ok(HttpResponse::Ok().json(json!({"result": "succes"})))
}
