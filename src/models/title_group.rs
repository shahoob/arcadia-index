use std::collections::HashMap;

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use super::master_group::UserCreatedMasterGroup;

// Every attribute is specific to the title, no specific information should be entered about the editions or the torrents
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct TitleGroup {
    pub id: i32,
    pub master_group: Option<i32>, // only if master groups are needed for this type of content
    pub name: String,
    pub name_aliases: Vec<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub created_by: i32,
    pub description: String,
    pub original_language: String,
    pub country_from: String,
    pub covers: Option<Vec<String>>,
    pub external_links: Vec<String>, // (public DBs, other trackers)
    pub embedded_links: Option<HashMap<String, String>>, // {name: link} (trailer, preview, etc.)
    // pub main_artists
    // pub artists_affiliated (multiple categories, multiple in each category) (composer, remixer, actors, developers, etc.)
    // pub entities_affiliated (multiple categories, mutliple in each category) (publisher, record label, franchise, etc.)
    pub category: i32, // ((movie: feature film, short film), (music: ep, album, compilation))
    pub public_ratings: Option<HashMap<String, String>>, // {service: rating}
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SimilarTitleGroups {
    pub group_1: i32,
    pub group_2: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserCreatedTitleGroup {
    pub name: String,
    pub name_aliases: Vec<String>,
    pub description: String,
    pub original_language: String,
    pub country_from: String,
    pub covers: Option<Vec<String>>,
    pub external_links: Vec<String>,
    pub embedded_links: Option<HashMap<String, String>>,
    // pub main_artists
    // pub artists_affiliated (multiple categories, multiple in each category) (composer, remixer, actors, developers, etc.)
    // pub entities_affiliated (multiple categories, mutliple in each category) (publisher, record label, franchise, etc.)
    pub category: i32, // ((movie: feature film, short film), (music: ep, album, compilation))
    pub public_ratings: Option<HashMap<String, String>>,
    // one of them should be given, if master groups are required for this type of content
    pub master_group_id: Option<i32>,
    pub master_group: Option<UserCreatedMasterGroup>,
}
