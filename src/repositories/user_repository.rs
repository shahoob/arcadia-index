use crate::models::user::{PublicUser, User};
use sqlx::PgPool;
use std::error::Error;

pub async fn find_user_by_username(pool: &PgPool, username: &str) -> Result<User, Box<dyn Error>> {
    let result = sqlx::query_as!(
        User,
        r#"
            SELECT * FROM users
            WHERE username = $1
        "#,
        username
    )
    .fetch_one(pool)
    .await;

    match result {
        Ok(_) => Ok(result.unwrap()),
        Err(e) => {
            match e {
                sqlx::Error::Database(db_error) => db_error.message().to_string(),
                _ => e.to_string(),
            };
            Err(format!("User not found").into())
        }
    }
}
pub async fn find_user_by_id(pool: &PgPool, id: &i64) -> Result<PublicUser, Box<dyn Error>> {
    // TODO: use id BIGINT PRIMARY KEY GENERATED ALWAYS AS DEFAULT
    let result = sqlx::query_as!(
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
                ratio,
                required_ratio,
                last_seen,
                class,
                forum_posts,
                forum_threads,
                group_comments,
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
                bonus_points
            FROM users
            WHERE id = $1
        "#,
        *id as i32
    )
    .fetch_one(pool)
    .await;

    match result {
        Ok(_) => Ok(result.unwrap()),
        Err(e) => {
            match e {
                sqlx::Error::Database(db_error) => db_error.message().to_string(),
                _ => e.to_string(),
            };
            Err(format!("User not found").into())
        }
    }
}
