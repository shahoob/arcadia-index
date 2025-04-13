use actix_files::NamedFile;
use actix_multipart::form::MultipartForm;
use actix_web::{HttpResponse, web};
use serde::Deserialize;
use utoipa::{IntoParams, ToSchema};

use crate::{
    Arcadia, Error, Result,
    models::{
        torrent::{TorrentSearch, UploadedTorrent},
        user::User,
    },
    repositories::torrent_repository::{create_torrent, search_torrents},
};

pub async fn upload_torrent(
    form: MultipartForm<UploadedTorrent>,
    arc: web::Data<Arcadia>,
    current_user: User,
) -> Result<HttpResponse> {
    // TODO : check if user can upload

    let torrent = create_torrent(
        &arc.pool,
        &form,
        &current_user,
        &arc.frontend_url,
        &arc.dottorrent_files_path,
    )
    .await?;

    Ok(HttpResponse::Created().json(torrent))
}

#[derive(Debug, Deserialize, IntoParams, ToSchema)]
pub struct DownloadTorrentQuery {
    id: i64,
}

#[utoipa::path(
    get,
    path = "/api/torrent",
    params (DownloadTorrentQuery),
    responses(
        (status = 200, description = "Successfully downloaded the torrent file"),
    )
)]
pub async fn download_dottorrent_file(
    query: web::Query<DownloadTorrentQuery>,
    arc: web::Data<Arcadia>,
    current_user: User,
) -> Result<NamedFile> {
    let file_path = &arc
        .dottorrent_files_path
        .join(query.id.to_string() + ".torrent");

    // should never happen as query.id is an int, but we never know...
    if !file_path.starts_with(&arc.dottorrent_files_path) {
        println!(
            "User(username: {}, id: {}) attempted path traversal: {:#?}",
            current_user.username,
            current_user.id,
            file_path.to_str()
        );
        return Err(Error::DottorrentFileNotFound);
    }

    actix_files::NamedFile::open(&file_path).map_err(|_| Error::DottorrentFileNotFound)
}

#[utoipa::path(
    get,
    path = "/api/search/torrent",
    responses(
        (status = 200, description = "Title groups and their torrents found"),
    )
)]
pub async fn find_torrents(
    form: web::Json<TorrentSearch>,
    arc: web::Data<Arcadia>,
) -> Result<HttpResponse> {
    let search_results = search_torrents(&arc.pool, &form).await?;

    Ok(HttpResponse::Ok().json(search_results))
}
