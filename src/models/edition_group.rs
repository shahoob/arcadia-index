use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{prelude::FromRow, types::Json};

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "source_enum")]
pub enum Source {
    CD,
    DVD5,
    DVD9,
    Vinyl,
    Web,
    Soundboard,
    SACD,
    DAT,
    Cassette,
    #[sqlx(rename = "Blu-Ray")]
    BluRay,
    LaserDisc,
    #[sqlx(rename = "HD-DVD")]
    HDDVD,
    HDTV,
    PDTV,
    TV,
    VHS,
    Mixed,
    #[sqlx(rename = "Physical-Book")]
    PhysicalBook,
}

// This represents encodes/transcodes of the same edition.
// All the torrents in it originate from the same source.
// It is independant people that produced multiple encodes/transcodes alongside the original one(s).
// Every attribute is specific to the edition, no information should be entered about the torrents or the title
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct EditionGroup {
    pub id: i32,
    pub title_group_id: i32,
    pub name: String, // edition name, not title name, (also, for Collections, includes the optional subscription level/tier)
    pub release_date: NaiveDateTime, // public release, (also, for Collections, date of the last (chronologically) item included)
    pub created_at: NaiveDateTime,   // database entry creation
    pub updated_at: NaiveDateTime,
    pub created_by_id: i32,
    pub description: Option<String>, // specific to the edition
    pub distributor: Option<String>, // web: [web stores/distributors], physical: [shop if specific edition ?]
    pub covers: Vec<String>,
    pub external_links: Vec<String>, // (public DBs, other trackers)
    pub source: Source,
    // this information will appea in the "title bar" of the edition
    // for collections : (date_from: first item date, first_item: numer/name of the first item, last_item: numer/name of the last item)
    pub additional_information: Option<Json<Value>>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct UserCreatedEditionGroup {
    pub name: String,
    pub release_date: NaiveDateTime,
    pub description: Option<String>,
    pub distributor: Option<String>,
    pub covers: Vec<String>,
    pub external_links: Vec<String>,
    pub source: Source,
    pub additional_information: Option<Json<Value>>,
    // one of them should be given
    pub title_group_id: Option<i32>,
    // pub title_group: Option<UserCreatedTitleGroup>,
}
