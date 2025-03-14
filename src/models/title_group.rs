use std::collections::HashMap;

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

// Every attribute is specific to the title, no specific information should be entered about the editions or the torrents
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct TitleGroup {
    pub id: i64,
    pub master_group: Option<i64>, // only if master groups are needed for this type of content
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
    pub group_1: i64,
    pub group_2: i64,
}
