use std::collections::HashSet;

use reqwest::Url;
pub mod api_doc;
pub mod handlers;
pub mod models;
pub mod repositories;
pub mod routes;
pub mod services;
pub mod tracker;

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum OpenSignups {
    Disabled,
    Enabled,
}

pub struct Arcadia {
    pub pool: sqlx::PgPool,
    pub jwt_secret: String,
    pub open_signups: OpenSignups,
    pub tracker_name: String,
    pub frontend_url: Url,
    pub tracker_url: Url,
    pub tracker_announce_interval: u32,
    pub allowed_torrent_clients: HashSet<Vec<u8>>,
    pub global_upload_factor: f64,
    pub global_download_factor: f64,
}

impl Arcadia {
    #[inline]
    pub fn is_open_signups(&self) -> bool {
        self.open_signups == OpenSignups::Enabled
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("database error: {0}")]
    GenericDatabaseError(#[from] sqlx::Error),

    #[error("account banned")]
    AccountBanned,

    #[error("could not create artist")]
    CouldNotCreateArtist(#[source] sqlx::Error),

    #[error("could not create artist affiliation")]
    CouldNotCreateArtistAffiliation(#[source] sqlx::Error),

    #[error("could not search for artists")]
    CouldNotSearchForArtists(#[source] sqlx::Error),

    #[error("could not create user")]
    CouldNotCreateUser(#[source] sqlx::Error),

    #[error("could not create edition group")]
    CouldNotCreateEditionGroup(#[source] sqlx::Error),

    #[error("could not create invitation")]
    CouldNotCreateInvitation(#[source] sqlx::Error),

    #[error("could not create master group")]
    CouldNotCreateMasterGroup(#[source] sqlx::Error),

    #[error("could not create notification")]
    CouldNotCreateNotification(#[source] sqlx::Error),

    #[error("could not create title group subscription")]
    CouldNotCreateTitleGroupSubscription(#[source] sqlx::Error),

    #[error("could not create title group subscription")]
    CouldNotCreateTitleGroupComment(#[source] sqlx::Error),

    #[error("could not create title group")]
    CouldNotCreateTitleGroup(#[source] sqlx::Error),

    #[error("could not create torrent")]
    CouldNotCreateTorrent(#[source] sqlx::Error),

    #[error("could not create torrent request")]
    CouldNotCreateTorrentRequest(#[source] sqlx::Error),

    #[error("could not create torrent request vote")]
    CouldNotCreateTorrentRequestVote(#[source] sqlx::Error),

    #[error("could not create torrent report")]
    CouldNotCreateTorrentReport(#[source] sqlx::Error),

    #[error("could not create series")]
    CouldNotCreateSeries(#[source] sqlx::Error),

    #[error("series with id '{0}' not found")]
    SeriesWithIdNotFound(i64),

    #[error("invalid invitation key")]
    InvitationKeyInvalid,

    #[error("invitation key required")]
    InvitationKeyRequired,

    #[error("invitation key already used")]
    InvitationKeyAlreadyUsed,

    #[error("no invitations available")]
    NoInvitationsAvailable,

    #[error("user '{0}' not found")]
    UserNotFound(String),

    #[error("user with id '{0}' not found")]
    UserWithIdNotFound(i64),

    #[error("wrong username or password")]
    WrongUsernameOrPassword,

    #[error("invalid or expired refresh token")]
    InvalidOrExpiredRefreshToken,

    #[error("unsupported notification type '{0}'")]
    UnsupportedNotification(String),

    #[error("unsupported subscription type '{0}'")]
    UnsupportedSubscription(String),

    #[error("not enough bonus points to place this bounty")]
    InsufficientBonusPointsForBounty,

    #[error("not enough upload to place this bounty")]
    InsufficientUploadForBounty,

    #[error("torrent file invalid")]
    TorrentFileInvalid,

    #[error("dottorrent file not found")]
    DottorrentFileNotFound,

    #[error("could not save torrent file in path: '{0}'\n'{1}'")]
    CouldNotSaveTorrentFile(String, String),

    #[error("error while searching for torrents: '{0}'")]
    ErrorSearchingForTorrents(String),

    #[error("error while deleting torrent: '{0}'")]
    ErrorDeletingTorrent(String),

    #[error("unexpected third party response")]
    UnexpectedThirdPartyResponse(#[from] reqwest::Error),

    #[error("not enough bonus points available")]
    NotEnoughBonusPointsAvailable,

    #[error("not enough freeleech tokens available")]
    NotEnoughFreeleechTokensAvailable,

    #[error("could not create gift")]
    CouldNotCreateGift(#[source] sqlx::Error),

    #[error("could not create forum post")]
    CouldNotCreateForumPost(#[source] sqlx::Error),

    #[error("could not create forum thread")]
    CouldNotCreateForumThread(#[source] sqlx::Error),

    #[error("could not find forum sub-category")]
    CouldNotFindForumSubCategory(#[source] sqlx::Error),

    #[error("could not find forum thread")]
    CouldNotFindForumThread(#[source] sqlx::Error),

    #[error("insufficient privileges")]
    InsufficientPrivileges,

    #[error("could not warn user: '{0}'")]
    CouldNotWarnUser(String),
}

pub type Result<T> = std::result::Result<T, Error>;

impl actix_web::ResponseError for Error {
    #[inline]
    fn status_code(&self) -> actix_web::http::StatusCode {
        actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
    }

    fn error_response(&self) -> actix_web::HttpResponse {
        actix_web::HttpResponse::build(self.status_code()).json(serde_json::json!({
            "error": format!("{self}"),
        }))
    }
}
