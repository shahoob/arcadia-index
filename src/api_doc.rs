use utoipa::OpenApi;

use crate::{
    handlers::{
        artist_handler::GetArtistPublicationsQuery, auth_handler::RegisterQuery,
        torrent_handler::DownloadTorrentQuery,
    },
    models::{
        artist::Artist,
        edition_group::EditionGroup,
        invitation::{Invitation, SentInvitation},
        master_group::{MasterGroup, UserCreatedMasterGroup},
        series::{Series, UserCreatedSeries},
        title_group::AffiliatedArtist,
        title_group_comment::{TitleGroupComment, UserCreatedTitleGroupComment},
        torrent::TorrentSearch,
        torrent_request::{TorrentRequest, UserCreatedTorrentRequest},
        torrent_request_vote::{TorrentRequestVote, UserCreatedTorrentRequestVote},
        user::{Login, PublicUser, Register, User},
    },
};

#[derive(OpenApi)]
#[openapi(
    info(title = "arcadia-index API",),
    paths(
        crate::handlers::auth_handler::register,
        crate::handlers::auth_handler::login,
        crate::handlers::artist_handler::get_artist_publications,
        crate::handlers::artist_handler::add_artist,
        crate::handlers::artist_handler::add_affiliated_artists,
        crate::handlers::torrent_handler::download_dottorrent_file,
        crate::handlers::torrent_handler::find_torrents,
        crate::handlers::edition_group_handler::add_edition_group,
        crate::handlers::invitation_handler::send_invitation,
        crate::handlers::master_group_handler::add_master_group,
        crate::handlers::series_handler::add_series,
        crate::handlers::series_handler::get_series,
        crate::handlers::subscriptions_handler::add_subscription,
        crate::handlers::subscriptions_handler::remove_subscription,
        crate::handlers::title_group_comment_handler::add_title_group_comment,
        crate::handlers::title_group_handler::add_title_group,
        crate::handlers::title_group_handler::get_title_group,
        crate::handlers::title_group_handler::get_lite_title_group_info,
        crate::handlers::torrent_request_handler::add_torrent_request,
        crate::handlers::torrent_request_vote_handler::add_torrent_request_vote,
        crate::handlers::user_handler::get_user,
        crate::handlers::user_handler::get_me
    ),
    components(schemas(
        Register,
        RegisterQuery,
        Login,
        GetArtistPublicationsQuery,
        DownloadTorrentQuery,
        TorrentSearch,
        Artist,
        AffiliatedArtist,
        User,
        PublicUser,
        EditionGroup,
        Invitation,
        SentInvitation,
        MasterGroup,
        UserCreatedMasterGroup,
        Series,
        UserCreatedSeries,
        UserCreatedTitleGroupComment,
        TitleGroupComment,
        TorrentRequest,
        UserCreatedTorrentRequest,
        TorrentRequestVote,
        UserCreatedTorrentRequestVote
    ),)
)]
pub struct ApiDoc;
