use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, Serialize, FromRow, ToSchema)]
pub struct Artist {
    pub id: i64,
    pub name: String,
    #[schema(value_type = String, format = DateTime)]
    pub created_at: NaiveDateTime,
    pub created_by_id: i64,
    pub description: String,
    pub pictures: Option<Vec<String>>,
    pub title_groups_amount: i32,
    pub edition_groups_amount: i32,
    pub torrents_amount: i32,
    pub seeders_amount: i32,
    pub leechers_amount: i32,
    pub snatches_amount: i32,
}

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct SimilarArtists {
    pub artist_1_id: i64,
    pub artist_2_id: i64,
}

#[derive(Debug, Deserialize, FromRow, ToSchema)]
pub struct UserCreatedArtist {
    pub name: String,
    pub description: String,
    pub pictures: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize, FromRow, ToSchema)]
pub struct ArtistLite {
    pub id: i64,
    pub name: String,
    pub pictures: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type, ToSchema)]
#[sqlx(type_name = "artist_role_enum")]
pub enum ArtistRole {
    // general roles
    #[sqlx(rename = "main")]
    #[serde(alias = "main")]
    Main,
    #[sqlx(rename = "producer")]
    #[serde(alias = "producer")]
    Producer,
    // music roles
    #[sqlx(rename = "guest")]
    #[serde(alias = "guest")]
    Guest,
    #[sqlx(rename = "composer")]
    #[serde(alias = "composer")]
    Composer,
    #[sqlx(rename = "conductor")]
    #[serde(alias = "conductor")]
    Conductor,
    #[sqlx(rename = "dj_compiler")]
    #[serde(alias = "dj_compiler")]
    DjCompiler,
    #[sqlx(rename = "remixer")]
    #[serde(alias = "remixer")]
    Remixer,
    #[sqlx(rename = "arranger")]
    #[serde(alias = "arranger")]
    Arranger,
    // movie/tv roles
    #[sqlx(rename = "director")]
    #[serde(alias = "director")]
    Director,
    #[sqlx(rename = "cinematographer")]
    #[serde(alias = "cinematographer")]
    Cinematographer,
    #[sqlx(rename = "actor")]
    #[serde(alias = "actor")]
    Actor,
    // book roles
    #[sqlx(rename = "author")]
    #[serde(alias = "author")]
    Author,
}

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct AffiliatedArtist {
    pub title_group_id: i64,
    pub artist_id: i64,
    pub roles: Vec<ArtistRole>,
    pub nickname: Option<String>, // for example: name of the character the actor is playing
    #[schema(value_type = String, format = DateTime)]
    pub created_at: NaiveDateTime,
    pub created_by_id: i64,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UserCreatedAffiliatedArtist {
    pub title_group_id: i64,
    pub artist_id: i64,
    pub roles: Vec<ArtistRole>,
    pub nickname: Option<String>,
}
