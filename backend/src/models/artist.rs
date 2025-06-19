use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use utoipa::ToSchema;

use super::title_group::TitleGroupHierarchyLite;

#[derive(Debug, Deserialize, Serialize, FromRow, ToSchema)]
pub struct Artist {
    pub id: i64,
    pub name: String,
    #[schema(value_type = String, format = DateTime)]
    pub created_at: DateTime<Utc>,
    pub created_by_id: i64,
    pub description: String,
    pub pictures: Vec<String>,
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
    pub pictures: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, FromRow, ToSchema)]
pub struct ArtistLite {
    pub id: i64,
    pub name: String,
    pub pictures: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type, ToSchema)]
#[sqlx(type_name = "artist_role_enum")]
pub enum ArtistRole {
    // general roles
    #[sqlx(rename = "main")]
    #[serde(rename = "main")]
    Main,
    #[sqlx(rename = "producer")]
    #[serde(rename = "producer")]
    Producer,
    // music roles
    #[sqlx(rename = "guest")]
    #[serde(rename = "guest")]
    Guest,
    #[sqlx(rename = "composer")]
    #[serde(rename = "composer")]
    Composer,
    #[sqlx(rename = "conductor")]
    #[serde(rename = "conductor")]
    Conductor,
    #[sqlx(rename = "dj_compiler")]
    #[serde(rename = "dj_compiler")]
    DjCompiler,
    #[sqlx(rename = "remixer")]
    #[serde(rename = "remixer")]
    Remixer,
    #[sqlx(rename = "arranger")]
    #[serde(rename = "arranger")]
    Arranger,
    // movie/tv roles
    #[sqlx(rename = "director")]
    #[serde(rename = "director")]
    Director,
    #[sqlx(rename = "cinematographer")]
    #[serde(rename = "cinematographer")]
    Cinematographer,
    #[sqlx(rename = "actor")]
    #[serde(rename = "actor")]
    Actor,
    // book roles
    #[sqlx(rename = "author")]
    #[serde(rename = "author")]
    Author,
}

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct AffiliatedArtist {
    pub title_group_id: i64,
    pub artist_id: i64,
    pub roles: Vec<ArtistRole>,
    pub nickname: Option<String>, // for example: name of the character the actor is playing
    #[schema(value_type = String, format = DateTime)]
    pub created_at: DateTime<Utc>,
    pub created_by_id: i64,
}

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct AffiliatedArtistLite {
    pub artist_id: i64,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserCreatedAffiliatedArtist {
    pub title_group_id: i64,
    pub artist_id: i64,
    pub roles: Vec<ArtistRole>,
    pub nickname: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct ArtistAndTitleGroupsLite {
    // used for the API doc, but not sure why it's considered dead code
    #[allow(dead_code)]
    pub artist: Artist,
    #[allow(dead_code)]
    pub title_groups: Vec<TitleGroupHierarchyLite>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct AffiliatedArtistHierarchy {
    pub title_group_id: i64,
    pub artist_id: i64,
    pub roles: Vec<ArtistRole>,
    pub nickname: Option<String>,
    #[schema(value_type = String, format = DateTime)]
    pub created_at: DateTime<Utc>,
    pub created_by_id: i64,
    pub artist: Artist,
}
