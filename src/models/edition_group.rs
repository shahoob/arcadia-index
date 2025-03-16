use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

// This represents encodes/transcodes of the same edition.
// All the torrents in it originate from the same source.
// It is independant people that produced multiple encodes/transcodes alongside the original one(s).
// Every attribute is specific to the edition, no information should be entered about the torrents or the title
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct EditionGroup {
    pub id: i32,
    pub title_group: i32,
    pub name: String,                // edition name, not title name
    pub release_date: NaiveDateTime, // public release
    pub created_at: NaiveDateTime,   // database entry creation
    pub updated_at: NaiveDateTime,
    pub created_by: i32,
    pub description: Option<String>, // specific to the edition
    pub distributor: Option<String>, // web: [web stores/distributors], physical: [shop if specific edition ?]
    pub covers: Vec<String>,
    pub external_links: Vec<String>, // (public DBs, other trackers)
    pub language: Option<String>,    // (fallback to original language) (english, french, etc.)
    pub source: String, //(dvd, web, bluray, hdtv, scanned book, digital book, cd, vinyl etc.)
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct UserCreatedEditionGroup {
    pub name: String,
    pub release_date: NaiveDateTime,
    pub description: Option<String>,
    pub distributor: Option<String>,
    pub covers: Vec<String>,
    pub external_links: Vec<String>,
    pub language: Option<String>,
    pub source: String,
    // one of them should be given
    pub title_group_id: Option<i32>,
    // pub title_group: Option<UserCreatedTitleGroup>,
}
