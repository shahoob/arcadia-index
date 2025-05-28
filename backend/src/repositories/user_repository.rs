use crate::{
    Error, Result,
    models::user::{PublicUser, UserCreatedUserWarning, UserWarning},
};
use sqlx::PgPool;

pub async fn find_user_by_id(pool: &PgPool, id: &i64) -> Result<PublicUser> {
    sqlx::query_as!(
        PublicUser,
        r#"
            SELECT
                id,
                username,
                avatar,
                created_at,
                description,
                uploaded,
                downloaded,
                real_uploaded,
                real_downloaded,
                ratio,
                required_ratio,
                last_seen,
                class,
                forum_posts,
                forum_threads,
                torrent_comments,
                request_comments,
                artist_comments,
                seeding,
                leeching,
                snatched,
                seeding_size,
                requests_filled,
                collages_started,
                requests_voted,
                average_seeding_time,
                invited,
                invitations,
                bonus_points,
                warned,
                banned
            FROM users
            WHERE id = $1
        "#,
        *id
    )
    .fetch_one(pool)
    .await
    .map_err(|_| Error::UserWithIdNotFound(*id))
}

pub async fn update_last_seen(pool: &PgPool, id: i64) -> Result<()> {
    let _ = sqlx::query!(
        r#"
            UPDATE users
            SET last_seen = NOW()
            WHERE id = $1
        "#,
        id
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn create_user_warning(
    pool: &PgPool,
    current_user_id: i64,
    user_warning: &UserCreatedUserWarning,
) -> Result<UserWarning> {
    let mut tx = pool.begin().await?;

    let _ = sqlx::query!(
        r#"
            UPDATE users
            SET warned = true,
            banned = CASE
                WHEN $2 IS TRUE THEN TRUE
                ELSE banned
            END
            WHERE id = $1
        "#,
        user_warning.user_id,
        user_warning.ban
    )
    .execute(&mut *tx)
    .await?;

    let user_warning = sqlx::query_as!(
        UserWarning,
        r#"
            INSERT INTO user_warnings (user_id, expires_at, reason, created_by_id, ban)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING *
        "#,
        user_warning.user_id,
        user_warning.expires_at,
        user_warning.reason,
        current_user_id,
        user_warning.ban
    )
    .fetch_one(&mut *tx)
    .await
    .map_err(Error::CouldNotCreateGift)?;

    tx.commit().await?;

    Ok(user_warning)
}

pub async fn find_user_warnings(pool: &PgPool, user_id: i64) -> Vec<UserWarning> {
    sqlx::query_as!(
        UserWarning,
        r#"
            SELECT * FROM user_warnings
            WHERE user_id = $1
        "#,
        user_id
    )
    .fetch_all(pool)
    .await
    .expect("failed to get user warnings")
}

pub async fn is_user_banned(pool: &PgPool, user_id: i64) -> bool {
    let banned = sqlx::query_scalar!("SELECT banned FROM users WHERE id = $1", user_id)
        .fetch_optional(pool)
        .await
        .expect("user not found");

    banned.unwrap()
}
