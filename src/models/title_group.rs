use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use sqlx::{prelude::FromRow, types::Json};

use super::edition_group::LiteEditionGroupHierachy;

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "content_type_enum")]
pub enum ContentType {
    Movie,
    #[sqlx(rename = "TV-Show")]
    TVShow,
    Music,
    Software,
    Book,
    // aka SiteRip, but also includes packs of other content than website dumps (books, etc.)
    // this allows users to group content (when possible), which lowers the load on the tracker and makes the upload process faster (1 announce instead of multiple)
    Collection,
}

// this is not to store the genre, but the format
#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "category_enum")]
pub enum Category {
    //music
    Ep,
    Album, // includes "live album" as an "edition"
    Single,
    Soundtrack,
    Anthology,
    Compilation,
    #[sqlx(rename = "Single")]
    SingleCategory,
    Remix,
    Bootleg,
    Mixtape,
    ConcertRecording,
    DjMix,
    // movies
    FeatureFilm,
    ShortFilm,
    // tv shows
    // none ?
    // software
    Game,
    Program,
    // written documents (books and their derivatives)
    Illustrated, // includes mangas, comics, visual novels
    Periodical,  //includes newspapers, magazines
    Book,        // hardcover, etc
    Article,     // includes studies, theseis, essais, etc.
    Manual,      // includes guides, music sheets, etc. for physical and digital products
    // should be used as little as possible
    Other,
}

// Every attribute is specific to the title,
// no specific information should be entered about the editions or the torrents
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct TitleGroup {
    pub id: i64,
    pub master_group_id: Option<i64>, // only if master groups are needed for this type of content
    pub name: String,
    pub name_aliases: Vec<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub created_by_id: i64,
    pub description: String,
    pub original_language: Option<String>,
    pub original_release_date: NaiveDateTime,
    pub tagline: Option<String>, // catchy sentence that represents the general idea of the title
    pub country_from: Option<String>,
    pub covers: Option<Vec<String>>,
    pub external_links: Vec<String>, // (public DBs, other trackers)
    pub embedded_links: Option<Json<Value>>, // {name: link} (trailer, preview, etc.)
    // pub main_artists
    // pub artists_affiliated (multiple categories, multiple in each category) (composer, remixer, actors, developers, etc.)
    // pub entities_affiliated (multiple categories, mutliple in each category) (publisher, record label, franchise, etc.)
    pub category: Option<Category>, // ((movie: feature film, short film), (music: ep, album, compilation))
    pub content_type: ContentType,  // movies, tv shows, books, games, etc
    pub tags: Vec<String>,
    pub public_ratings: Option<Json<Value>>, // {service: rating}
    pub series_id: Option<i64>,
    // pub edition_groups: Option<Vec<EditionGroup>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SimilarTitleGroups {
    pub group_1: i64,
    pub group_2: i64,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct AffiliatedArtist {
    pub title_group_id: i64,
    pub artist_id: i64,
    pub status: String,
    pub nickname: Option<String>, // for example: name of the character the actor is playing
    pub created_at: NaiveDateTime,
    pub created_by_id: i64,
}

#[derive(Debug, Deserialize)]
pub struct UserCreatedAffiliatedArtist {
    pub title_group_id: i64,
    pub artist_id: i64,
    pub status: String,
    pub nickname: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
pub struct UserCreatedTitleGroup {
    pub name: String,
    pub name_aliases: Vec<String>,
    pub description: String,
    pub original_language: Option<String>,
    pub country_from: Option<String>,
    pub covers: Option<Vec<String>>,
    pub external_links: Vec<String>,
    pub embedded_links: Option<Json<Value>>,
    // pub artists_affiliated: //(multiple categories, multiple in each category) (composer, remixer, actors, developers, etc.)
    // pub entities_affiliated (multiple categories, mutliple in each category) (publisher, record label, franchise, etc.)
    pub category: Option<Category>, // ((movie: feature film, short film), (music: ep, album, compilation))
    pub content_type: ContentType,  // movies, tv shows, books, games, etc
    pub tags: Vec<String>,
    pub tagline: Option<String>,
    pub original_release_date: NaiveDateTime,
    pub affiliated_artists: Vec<Json<Value>>,
    pub series_id: Option<i64>,
    // one of them should be given, if master groups are required for this type of content
    pub master_group_id: Option<i64>,
    // pub master_group: Option<UserCreatedMasterGroup>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct LiteTitleGroupHierachy {
    pub name: String,
    pub covers: Option<Vec<String>>,
    pub category: Option<Category>, // ((movie: feature film, short film), (music: ep, album, compilation))
    pub content_type: ContentType,  // movies, tv shows, books, games, etc
    pub tags: Vec<String>,
    pub original_release_date: NaiveDateTime,
    pub affiliated_artists: Vec<Json<Value>>,
    pub editions: Vec<LiteEditionGroupHierachy>,
}

pub fn create_default_title_group() -> UserCreatedTitleGroup {
    UserCreatedTitleGroup {
        name: String::from("Untitled"),
        name_aliases: Vec::new(),
        description: String::from("No description provided"),
        original_language: None,
        country_from: None,
        covers: Some(vec!["".to_string()]),
        external_links: vec!["".to_string()],
        embedded_links: Some(Json(json!({}))),
        category: None,
        content_type: ContentType::Book,
        tags: Vec::new(),
        tagline: None,
        original_release_date: NaiveDate::parse_from_str("2000-01-01", "%Y-%m-%d")
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap(),
        affiliated_artists: Vec::new(),
        series_id: None,
        master_group_id: None,
    }
}
