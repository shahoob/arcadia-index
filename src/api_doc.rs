use utoipa::OpenApi;

use crate::{
    handlers::{
        artist_handler::GetArtistPublicationsQuery, auth_handler::RegisterQuery,
        torrent_handler::DownloadTorrentQuery,
    },
    models::user::{Login, Register},
};

#[derive(OpenApi)]
#[openapi(
    info(title = "arcadia-index API",),
    paths(
        crate::handlers::auth_handler::register,
        crate::handlers::auth_handler::login,
        crate::handlers::artist_handler::get_artist_publications,
        crate::handlers::torrent_handler::download_dottorrent_file,
    ),
    components(schemas(
        Register,
        RegisterQuery,
        Login,
        GetArtistPublicationsQuery,
        DownloadTorrentQuery
    ),)
)]
pub struct ApiDoc;
