use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use sqlx::{prelude::FromRow, types::Json};
use utoipa::ToSchema;

use super::{
    artist::AffiliatedArtistHierarchy,
    edition_group::{EditionGroupHierarchy, EditionGroupHierarchyLite, EditionGroupInfoLite},
    series::SeriesLite,
    title_group_comment::TitleGroupCommentHierarchy,
    torrent_request::TorrentRequest,
};

#[derive(Debug, Serialize, Deserialize, sqlx::Type, ToSchema)]
#[sqlx(type_name = "content_type_enum")]
pub enum ContentType {
    #[sqlx(rename = "movie")]
    #[serde(rename = "movie")]
    Movie,
    #[sqlx(rename = "tv_show")]
    #[serde(rename = "tv_show")]
    TVShow,
    #[sqlx(rename = "music")]
    #[serde(rename = "music")]
    Music,
    #[sqlx(rename = "software")]
    #[serde(rename = "software")]
    Software,
    #[sqlx(rename = "book")]
    #[serde(rename = "book")]
    Book,
    // aka SiteRip, but also includes packs of other content than website dumps (books, etc.)
    // this allows users to group content (when possible), which lowers the load on the tracker and makes the upload process faster (1 announce instead of multiple)
    #[sqlx(rename = "collection")]
    #[serde(rename = "collection")]
    Collection,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type, ToSchema)]
#[sqlx(type_name = "platform_enum")]
pub enum Platform {
    Windows,
    Linux,
    MacOS,
    Xbox,
}

// this is not to store the genre, but the format
#[derive(Debug, Serialize, Deserialize, sqlx::Type, ToSchema)]
#[sqlx(type_name = "title_group_category_enum")]
pub enum TitleGroupCategory {
    //music
    Ep,
    Album, // includes "live album" as an "edition"
    Single,
    Soundtrack,
    Anthology,
    Compilation,
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
#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct TitleGroup {
    pub id: i64,
    pub master_group_id: Option<i64>, // only if master groups are needed for this type of content
    pub name: String,
    pub name_aliases: Vec<String>,
    #[schema(value_type = String, format = DateTime)]
    pub created_at: NaiveDateTime,
    #[schema(value_type = String, format = DateTime)]
    pub updated_at: NaiveDateTime,
    pub created_by_id: i64,
    pub description: String,
    pub platform: Option<Platform>,
    pub original_language: Option<String>,
    #[schema(value_type = String, format = DateTime)]
    pub original_release_date: NaiveDateTime,
    pub tagline: Option<String>, // catchy sentence that represents the general idea of the title
    pub country_from: Option<String>,
    pub covers: Vec<String>,
    pub external_links: Vec<String>, // (public DBs, other trackers)
    #[schema(value_type = HashMap<String, String>)]
    pub embedded_links: Json<Value>, // {name: link} (trailer, preview, etc.)
    pub category: Option<TitleGroupCategory>, // ((movie: feature film, short film), (music: ep, album, compilation))
    pub content_type: ContentType,            // movies, tv shows, books, games, etc
    pub tags: Vec<String>,
    #[schema(value_type = HashMap<String, String>)]
    pub public_ratings: Option<Json<Value>>, // {service: rating}
    pub series_id: Option<i64>,
    pub screenshots: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SimilarTitleGroups {
    pub group_1: i64,
    pub group_2: i64,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type, ToSchema)]
pub struct UserCreatedTitleGroup {
    pub name: String,
    pub name_aliases: Vec<String>,
    pub description: String,
    pub original_language: Option<String>,
    pub country_from: Option<String>,
    pub covers: Vec<String>,
    pub external_links: Vec<String>,
    #[schema(value_type = HashMap<String, String>)]
    pub embedded_links: Json<Value>,
    // pub artists_affiliated: //(multiple categories, multiple in each category) (composer, remixer, actors, developers, etc.)
    // pub entities_affiliated (multiple categories, mutliple in each category) (publisher, record label, franchise, etc.)
    pub category: Option<TitleGroupCategory>, // ((movie: feature film, short film), (music: ep, album, compilation))
    pub content_type: ContentType,            // movies, tv shows, books, games, etc
    pub tags: Vec<String>,
    pub tagline: Option<String>,
    pub platform: Option<Platform>,
    #[schema(value_type = String, format = DateTime)]
    pub original_release_date: NaiveDateTime,
    #[schema(value_type = HashMap<String, String>)]
    pub affiliated_artists: Vec<Json<Value>>,
    pub series_id: Option<i64>,
    pub screenshots: Vec<String>,
    // one of them should be given, if master groups are required for this type of content
    pub master_group_id: Option<i64>,
    // pub master_group: Option<UserCreatedMasterGroup>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct TitleGroupHierarchyLite {
    pub id: i64,
    pub name: String,
    pub covers: Vec<String>,
    pub category: Option<TitleGroupCategory>,
    pub content_type: ContentType,
    pub tags: Vec<String>,
    #[schema(value_type = String, format = DateTime)]
    pub original_release_date: NaiveDateTime,
    // #[schema(value_type = HashMap<String, String>)]
    // pub affiliated_artists: Vec<Json<Value>>,
    pub edition_groups: Vec<EditionGroupHierarchyLite>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct TitleGroupLite {
    pub id: i64,
    pub name: String,
    pub content_type: ContentType,
    pub edition_groups: Vec<EditionGroupInfoLite>,
    pub platform: Option<Platform>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct TitleGroupInfoLite {
    pub id: i64,
    pub name: String,
    pub content_type: ContentType,
}

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct TitleGroupHierarchy {
    pub id: i64,
    pub master_group_id: Option<i64>,
    pub name: String,
    pub name_aliases: Vec<String>,
    #[schema(value_type = String, format = DateTime)]
    pub created_at: NaiveDateTime,
    #[schema(value_type = String, format = DateTime)]
    pub updated_at: NaiveDateTime,
    pub created_by_id: i64,
    pub description: String,
    pub platform: Option<Platform>,
    pub original_language: Option<String>,
    #[schema(value_type = String, format = DateTime)]
    pub original_release_date: NaiveDateTime,
    pub tagline: Option<String>,
    pub country_from: Option<String>,
    pub covers: Vec<String>,
    pub external_links: Vec<String>,
    #[schema(value_type = HashMap<String, String>)]
    pub embedded_links: Json<Value>,
    pub category: Option<TitleGroupCategory>,
    pub content_type: ContentType,
    pub tags: Vec<String>,
    #[schema(value_type = HashMap<String, String>)]
    pub public_ratings: Option<Json<Value>>,
    pub series_id: Option<i64>,
    pub screenshots: Vec<String>,
    pub edition_groups: Vec<EditionGroupHierarchy>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct TitleGroupAndAssociatedData {
    pub id: i64,
    pub master_group_id: Option<i64>,
    pub name: String,
    pub name_aliases: Vec<String>,
    #[schema(value_type = String, format = DateTime)]
    pub created_at: NaiveDateTime,
    #[schema(value_type = String, format = DateTime)]
    pub updated_at: NaiveDateTime,
    pub created_by_id: i64,
    pub description: String,
    pub platform: Option<Platform>,
    pub original_language: Option<String>,
    #[schema(value_type = String, format = DateTime)]
    pub original_release_date: NaiveDateTime,
    pub tagline: Option<String>,
    pub country_from: Option<String>,
    pub covers: Vec<String>,
    pub external_links: Vec<String>,
    #[schema(value_type = HashMap<String, String>)]
    pub embedded_links: Json<Value>,
    pub category: Option<TitleGroupCategory>,
    pub content_type: ContentType,
    pub tags: Vec<String>,
    #[schema(value_type = HashMap<String, String>)]
    pub public_ratings: Option<Json<Value>>,
    pub series_id: Option<i64>,
    pub screenshots: Vec<String>,
    pub edition_groups: Vec<EditionGroupHierarchy>,
    pub series: SeriesLite,
    pub affiliated_artists: Vec<AffiliatedArtistHierarchy>,
    pub title_group_comments: Vec<TitleGroupCommentHierarchy>,
    pub torrent_requests: Vec<TorrentRequest>,
    pub is_subscribed: bool,
    pub in_same_master_group: Vec<TitleGroupLite>,
}

pub fn create_default_title_group() -> UserCreatedTitleGroup {
    UserCreatedTitleGroup {
        name: String::from("Untitled"),
        name_aliases: Vec::new(),
        description: String::from("No description provided"),
        original_language: None,
        country_from: None,
        covers: vec!["".to_string()],
        external_links: vec!["".to_string()],
        embedded_links: Json(json!({})),
        category: None,
        content_type: ContentType::Book,
        tags: Vec::new(),
        tagline: None,
        platform: None,
        original_release_date: NaiveDate::parse_from_str("2000-01-01", "%Y-%m-%d")
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap(),
        affiliated_artists: Vec::new(),
        series_id: None,
        screenshots: Vec::new(),
        master_group_id: None,
    }
}
