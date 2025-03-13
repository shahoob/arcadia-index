use std::{net::IpAddr, time::Duration};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: u32,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub registered_from_ip: IpAddr,
    pub created_at: chrono::NaiveDateTime,
    pub description: String,
    pub uploaded: u64,
    pub downloaded: u64,
    pub ratio: f64,
    pub required_ratio: f64,
    pub last_seen: chrono::NaiveDateTime,
    pub class: String,
    pub forum_posts: u32,
    pub forum_threads: u32,
    pub group_comments: u32,
    pub torrent_comments: u32,
    pub request_comments: u32,
    pub artist_comments: u64,
    pub seeding: u32,
    pub leeching: u32,
    pub snatched: u32,
    pub seeding_size: u64,
    pub requests_filled: u64,
    pub collages_started: u64,
    pub requests_voted: u64,
    pub average_seeding_time: Duration,
    pub invited: u64,
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
    pub email: String,
    pub password: String,
    pub remember_me: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}
