use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{prelude::FromRow, types::Json};
use utoipa::ToSchema;

use super::torrent::LiteTorrent;

#[derive(Debug, Serialize, Deserialize, ToSchema, sqlx::Type)]
#[sqlx(type_name = "source_enum")]
pub enum Source {
    #[sqlx(rename = "CD")]
    #[serde(alias = "CD")]
    Cd,
    #[sqlx(rename = "DVD5")]
    #[serde(alias = "DVD5")]
    Dvd5,
    #[sqlx(rename = "DVD9")]
    #[serde(alias = "DVD9")]
    Dvd9,
    Vinyl,
    Web,
    Soundboard,
    #[sqlx(rename = "SACD")]
    #[serde(alias = "SACD")]
    Sacd,
    #[sqlx(rename = "DAT")]
    #[serde(alias = "DAT")]
    Dat,
    Cassette,
    #[sqlx(rename = "Blu-Ray")]
    #[serde(alias = "Blu-Ray")]
    BluRay,
    LaserDisc,
    #[sqlx(rename = "HD-DVD")]
    #[serde(alias = "HD-DVD")]
    Hddvd,
    #[sqlx(rename = "HDTV")]
    #[serde(alias = "HDTV")]
    Hdtv,
    #[sqlx(rename = "PDTV")]
    #[serde(alias = "PDTV")]
    Pdtv,
    #[sqlx(rename = "TV")]
    #[serde(alias = "TV")]
    Tv,
    #[sqlx(rename = "VHS")]
    #[serde(alias = "VHS")]
    Vhs,
    Mixed,
    #[sqlx(rename = "Physical Book")]
    #[serde(alias = "Physical Book")]
    PhysicalBook,
}

// This represents encodes/transcodes of the same edition.
// All the torrents in it originate from the same source.
// It is independant people that produced multiple encodes/transcodes alongside the original one(s).
// Every attribute is specific to the edition, no information should be entered about the torrents or the title
#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct EditionGroup {
    pub id: i64,
    pub title_group_id: i64,
    pub name: String, // edition name, not title name, (also, for Collections, includes the optional subscription level/tier)
    #[schema(value_type = String, format = DateTime)]
    pub release_date: NaiveDateTime, // public release, (also, for Collections, date of the last (chronologically) item included)
    #[schema(value_type = String, format = DateTime)]
    pub created_at: NaiveDateTime, // database entry creation
    #[schema(value_type = String, format = DateTime)]
    pub updated_at: NaiveDateTime,
    pub created_by_id: i64,
    pub description: Option<String>, // specific to the edition
    pub distributor: Option<String>, // web: [web stores/distributors], physical: [shop if specific edition ?]
    pub covers: Vec<String>,
    pub external_links: Vec<String>, // (public DBs, other trackers)
    pub source: Source,
    // this information will appea in the "title bar" of the edition
    // for collections : (date_from: first item date, first_item: numer/name of the first item, last_item: numer/name of the last item)
    #[schema(value_type = Value)]
    pub additional_information: Option<Json<Value>>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct UserCreatedEditionGroup {
    pub name: String,
    #[schema(value_type = String, format = DateTime)]
    pub release_date: NaiveDateTime,
    pub description: Option<String>,
    pub distributor: Option<String>,
    pub covers: Vec<String>,
    pub external_links: Vec<String>,
    pub source: Source,
    #[schema(value_type = Value)]
    pub additional_information: Option<Json<Value>>,
    // one of them should be given
    pub title_group_id: Option<i64>,
    // pub title_group: Option<UserCreatedTitleGroup>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct LiteEditionGroupHierachy {
    pub id: i64,
    pub title_group_id: i64,
    pub name: String,
    pub release_date: NaiveDateTime,
    pub distributor: Option<String>,
    pub covers: Vec<String>,
    pub source: Source,
    pub additional_information: Option<Json<Value>>,
    pub torrents: Vec<LiteTorrent>,
}
