use actix_multipart::form::MultipartForm;
use actix_web::{
    HttpResponse,
    http::header::{
        Charset, ContentDisposition, ContentType, DispositionParam, DispositionType, ExtendedValue,
    },
    web,
};
use serde::Deserialize;
use utoipa::{IntoParams, ToSchema};

use crate::{
    Arcadia, Result,
    models::{
        torrent::{Torrent, TorrentSearch, TorrentSearchResults, UploadedTorrent},
        user::User,
    },
    repositories::torrent_repository::{create_torrent, get_torrent, search_torrents},
};

#[utoipa::path(
    post,
    path = "/api/torrent",
    request_body(content = UploadedTorrent, content_type = "multipart/form-data"),
    responses(
        (status = 200, description = "Successfully uploaded the torrent", body=Torrent),
    )
)]
pub async fn upload_torrent(
    form: MultipartForm<UploadedTorrent>,
    arc: web::Data<Arcadia>,
    current_user: User,
) -> Result<HttpResponse> {
    // TODO : check if user can upload

    let torrent = create_torrent(&arc.pool, &form, &current_user).await?;

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
) -> Result<HttpResponse> {
    let torrent = get_torrent(
        &arc.pool,
        &current_user,
        query.id,
        &arc.tracker_name,
        arc.frontend_url.as_ref(),
        arc.tracker_url.as_ref(),
    )
    .await?;

    let cd = ContentDisposition {
        disposition: DispositionType::Attachment,
        parameters: vec![
            DispositionParam::FilenameExt(ExtendedValue {
                charset: Charset::Ext(String::from("UTF-8")),
                language_tag: None,
                value: format!("{}.torrent", torrent.title).into_bytes(),
            }),
            // TODO: add fallback for better compatibility
            //DispositionParam::Filename(format!("{}.torrent", name_ascii))
        ],
    };

    Ok(HttpResponse::Ok()
        .insert_header(ContentType::octet_stream())
        .insert_header(cd)
        .body(torrent.file_contents))
}

#[utoipa::path(
    get,
    path = "/api/search/torrent",
    responses(
        (status = 200, description = "Title groups and their torrents found", body=TorrentSearchResults),
    )
)]
pub async fn find_torrents(
    form: web::Json<TorrentSearch>,
    arc: web::Data<Arcadia>,
) -> Result<HttpResponse> {
    let search_results = search_torrents(&arc.pool, &form).await?;

    Ok(HttpResponse::Ok().json(search_results))
}
