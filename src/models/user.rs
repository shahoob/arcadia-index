use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub avatar: String,
    pub email: String,
    pub password_hash: String,
    pub registered_from_ip: String,
    pub created_at: NaiveDateTime,
    pub description: String,
    pub uploaded: i64,
    pub downloaded: i64,
    pub ratio: f64,
    pub required_ratio: f64,
    pub last_seen: NaiveDateTime,
    pub class: String,
    pub forum_posts: i32,
    pub forum_threads: i32,
    pub group_comments: i32,
    pub torrent_comments: i32,
    pub request_comments: i32,
    pub artist_comments: i64,
    pub seeding: i32,
    pub leeching: i32,
    pub snatched: i32,
    pub seeding_size: i64,
    pub requests_filled: i64,
    pub collages_started: i64,
    pub requests_voted: i64,
    pub average_seeding_time: i64, //in seconds
    pub invited: i64,
    pub invitations: i16,
    pub bonus_points: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Register {
    pub username: String,
    pub password: String,
    pub password_verify: String,
    pub email: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Login {
    pub username: String,
    pub password: String,
    pub remember_me: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i32,
    pub exp: usize,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct PublicUser {
    pub id: i32,
    pub username: String,
    pub avatar: String,
    pub created_at: NaiveDateTime,
    pub description: String,
    pub uploaded: i64,
    pub downloaded: i64,
    pub ratio: f64,
    pub required_ratio: f64,
    pub last_seen: NaiveDateTime,
    pub class: String,
    pub forum_posts: i32,
    pub forum_threads: i32,
    pub group_comments: i32,
    pub torrent_comments: i32,
    pub request_comments: i32,
    pub artist_comments: i64,
    pub seeding: i32,
    pub leeching: i32,
    pub snatched: i32,
    pub seeding_size: i64,
    pub requests_filled: i64,
    pub collages_started: i64,
    pub requests_voted: i64,
    pub average_seeding_time: i64,
    pub invited: i64,
    pub invitations: i16,
    pub bonus_points: i64,
}
