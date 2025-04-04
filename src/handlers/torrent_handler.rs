use actix_multipart::form::MultipartForm;
use actix_web::{HttpResponse, web};

use crate::{
    Arcadia, Result,
    models::{torrent::UploadedTorrent, user::User},
    repositories::torrent_repository::create_torrent,
};

pub async fn upload_torrent(
    form: MultipartForm<UploadedTorrent>,
    arc: web::Data<Arcadia>,
    current_user: User,
) -> Result<HttpResponse> {
    // TODO : check if user can upload

    let torrent = create_torrent(&arc.pool, &form, &current_user).await?;

    Ok(HttpResponse::Created().json(torrent))
}
