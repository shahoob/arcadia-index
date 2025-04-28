use sqlx::{PgPool, postgres::PgQueryResult};

use crate::handlers::announce_handler::Error;

#[derive(sqlx::FromRow)]
pub struct UserCompact {
    pub id: i64,
}

pub async fn find_user_with_passkey(
    pool: &PgPool,
    passkey_upper: i64,
    passkey_lower: i64,
) -> Result<UserCompact, Error> {
    sqlx::query_as!(
        UserCompact,
        r#"
            SELECT id FROM users
            WHERE (passkey_upper, passkey_lower) = ($1, $2)
        "#,
        passkey_upper,
        passkey_lower
    )
    .fetch_one(pool)
    .await
    .map_err(|_| Error::InvalidPassKey)
}

#[derive(sqlx::FromRow)]
pub struct TorrentCompact {
    pub id: i64,
    pub upload_factor: f64,
    pub download_factor: f64,
}

pub async fn find_torrent_with_id(
    pool: &PgPool,
    info_hash: &[u8; 20],
) -> Result<TorrentCompact, Error> {
    sqlx::query_as!(
        TorrentCompact,
        r#"
            SELECT id, upload_factor, download_factor FROM torrents
            WHERE info_hash = $1
        "#,
        info_hash
    )
    .fetch_one(pool)
    .await
    .map_err(|_| Error::InvalidInfoHash)
}

pub async fn credit_user_upload_download(
    pool: &PgPool,
    uploaded: i64,
    downloaded: i64,
    real_uploaded: i64,
    real_downloaded: i64,
    user_id: i64,
) -> Result<PgQueryResult, Error> {
    sqlx::query!(
        r#"
        UPDATE users
        SET uploaded = uploaded + $1,
            downloaded = downloaded + $2,
            real_uploaded = real_uploaded + $3,
            real_downloaded = real_downloaded + $4
        WHERE id = $5
        "#,
        uploaded,
        downloaded,
        real_uploaded,
        real_downloaded,
        user_id
    )
    .execute(pool)
    .await
    .map_err(|_| Error::InvalidUserId)
}
