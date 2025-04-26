use sqlx::PgPool;

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
}

pub async fn find_torrent_with_id(
    pool: &PgPool,
    info_hash: &[u8; 20],
) -> Result<TorrentCompact, Error> {
    sqlx::query_as!(
        TorrentCompact,
        r#"
            SELECT id FROM torrents
            WHERE info_hash = $1
        "#,
        info_hash
    )
    .fetch_one(pool)
    .await
    .map_err(|_| Error::InvalidInfoHash)
}
