use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{prelude::FromRow, types::Json};

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
}

// Every attribute is specific to the title,
// no specific information should be entered about the editions or the torrents
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct TitleGroup {
    pub id: i32,
    pub master_group_id: Option<i32>, // only if master groups are needed for this type of content
    pub name: String,
    pub name_aliases: Vec<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub created_by_id: i32,
    pub description: String,
    pub original_language: String,
    pub original_release_date: NaiveDateTime,
    pub tagline: Option<String>, // catchy sentence that represents the general idea of the title
    pub country_from: String,
    pub covers: Option<Vec<String>>,
    pub external_links: Vec<String>, // (public DBs, other trackers)
    pub embedded_links: Option<Json<Value>>, // {name: link} (trailer, preview, etc.)
    // pub main_artists
    // pub artists_affiliated (multiple categories, multiple in each category) (composer, remixer, actors, developers, etc.)
    // pub entities_affiliated (multiple categories, mutliple in each category) (publisher, record label, franchise, etc.)
    pub category: Category, // ((movie: feature film, short film), (music: ep, album, compilation))
    pub content_type: ContentType, // movies, tv shows, books, games, etc
    pub tags: Vec<String>,
    pub public_ratings: Option<Json<Value>>, // {service: rating}
    pub serie_id: Option<i32>,
    // pub edition_groups: Option<Vec<EditionGroup>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SimilarTitleGroups {
    pub group_1: i32,
    pub group_2: i32,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct AffiliatedArtist {
    pub title_group_id: i32,
    pub artist_id: i32,
    pub status: String,
    pub nickname: Option<String>, // for example: name of the character the actor is playing
    pub created_at: NaiveDateTime,
    pub created_by_id: i32,
}

#[derive(Debug, Deserialize)]
pub struct UserCreatedAffiliatedArtist {
    pub title_group_id: i32,
    pub artist_id: i32,
    pub status: String,
    pub nickname: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
pub struct UserCreatedTitleGroup {
    pub name: String,
    pub name_aliases: Vec<String>,
    pub description: String,
    pub original_language: String,
    pub country_from: String,
    pub covers: Option<Vec<String>>,
    pub external_links: Vec<String>,
    pub embedded_links: Option<Json<Value>>,
    // pub artists_affiliated: //(multiple categories, multiple in each category) (composer, remixer, actors, developers, etc.)
    // pub entities_affiliated (multiple categories, mutliple in each category) (publisher, record label, franchise, etc.)
    pub category: Category, // ((movie: feature film, short film), (music: ep, album, compilation))
    pub content_type: ContentType, // movies, tv shows, books, games, etc
    pub tags: Vec<String>,
    pub tagline: Option<String>,
    pub original_release_date: NaiveDateTime,
    pub affiliated_artists: Vec<Json<Value>>,
    pub serie: Option<i32>,
    // one of them should be given, if master groups are required for this type of content
    pub master_group_id: Option<i32>,
    // pub master_group: Option<UserCreatedMasterGroup>,
}
