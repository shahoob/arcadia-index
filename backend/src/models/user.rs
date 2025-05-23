use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use sqlx::types::ipnetwork::IpNetwork;
use utoipa::ToSchema;

use super::peer::Peer;

// TODO: deserialize the settings field to a rust struct, currently doesn't seem possible
// https://github.com/launchbadge/sqlx/issues/3153#issuecomment-2798756953
// #[derive(Serialize, Deserialize, Debug, sqlx::Type)]
// #[sqlx(type_name = "item_detail_layout")]
// pub enum ItemDetailLayout {
// #[sqlx(rename = "header")]
// #[serde(rename = "header")]
//     Header,
// #[sqlx(rename = "sidebar_right")]
// #[serde(rename = "sidebar_right")]
//     SideBarRight,
// #[sqlx(rename = "sidebar_left")]
// #[serde(rename = "sidebar_left")]
//     SideBarLeft,
// }

// #[derive(Serialize, Deserialize, Debug, FromRow, sqlx::Type)]
// pub struct SiteAppearanceSettings {
//     pub item_detail_layout: ItemDetailLayout,
// }

// #[derive(Serialize, Deserialize, Debug, FromRow, sqlx::Type)]
// // #[sqlx(type_name = "user_settings")]
// pub struct UserSettings {
//     pub site_appearance: SiteAppearanceSettings,
// }

// causes errors
// https://github.com/launchbadge/sqlx/issues/3869
// #[derive(Debug, Serialize, Deserialize, sqlx::Type, ToSchema)]
// #[sqlx(type_name = "user_class_enum")]
// pub enum UserClass {
//     #[sqlx(rename = "newbie")]
//     #[serde(rename = "newbie")]
//     Newbie,
//     #[sqlx(rename = "staff")]
//     #[serde(rename = "staff")]
//     Staff,
// }

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub avatar: Option<String>,
    pub email: String,
    pub password_hash: String,
    #[schema(value_type = String, format = "0.0.0.0")]
    pub registered_from_ip: IpNetwork,
    #[schema(value_type = String, format = DateTime)]
    pub created_at: DateTime<Local>,
    pub description: String,
    pub uploaded: i64,
    pub real_uploaded: i64,
    pub downloaded: i64,
    pub real_downloaded: i64,
    pub ratio: f64,
    pub required_ratio: f64,
    #[schema(value_type = String, format = DateTime)]
    pub last_seen: DateTime<Local>,
    pub class: String,
    pub forum_posts: i32,
    pub forum_threads: i32,
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
    pub freeleech_tokens: i32,
    pub settings: serde_json::Value,
    pub warned: bool,
    pub passkey_upper: i64,
    pub passkey_lower: i64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Register {
    pub username: String,
    pub password: String,
    pub password_verify: String,
    pub email: String,
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct Login {
    pub username: String,
    pub password: String,
    pub remember_me: bool,
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct LoginResponse {
    pub token: String,
    pub refresh_token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i64,
    pub exp: usize,
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct RefreshToken {
    pub refresh_token: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct PublicUser {
    pub id: i64,
    pub username: String,
    pub avatar: Option<String>,
    #[schema(value_type = String, format = DateTime)]
    pub created_at: DateTime<Local>,
    pub description: String,
    pub uploaded: i64,
    pub real_uploaded: i64,
    pub downloaded: i64,
    pub real_downloaded: i64,
    pub ratio: f64,
    pub required_ratio: f64,
    #[schema(value_type = String, format = DateTime)]
    pub last_seen: DateTime<Local>,
    pub class: String,
    pub forum_posts: i32,
    pub forum_threads: i32,
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
    pub warned: bool,
}

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct UserLite {
    pub id: i64,
    pub username: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct UserLiteAvatar {
    pub id: i64,
    pub username: String,
    pub avatar: Option<String>,
    pub warned: bool,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Profile {
    pub user: User,
    pub peers: Vec<Peer>,
    pub user_warnings: Vec<UserWarning>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PublicProfile {
    pub user: PublicUser,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, FromRow)]
pub struct UserWarning {
    pub id: i64,
    pub user_id: i64,
    #[schema(value_type = String, format = DateTime)]
    pub created_at: DateTime<Local>,
    #[schema(value_type = String, format = DateTime)]
    pub expires_at: DateTime<Local>,
    pub reason: String,
    pub created_by_id: i64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserCreatedUserWarning {
    pub user_id: i64,
    #[schema(value_type = String, format = DateTime)]
    pub expires_at: DateTime<Local>,
    pub reason: String,
}
