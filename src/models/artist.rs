use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct Artist {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub pictures: Option<Vec<String>>,
    pub title_groups_amount: i32,
    pub edition_groups_amount: i32,
    pub torrents_amount: i32,
    pub seeders_amount: i32,
    pub leechers_amount: i32,
    pub snatches_amount: i32,
}

#[derive(Debug, Deserialize, FromRow)]
pub struct SimilarArtists {
    pub artist_1: i32,
    pub artist_2: i32,
}

#[derive(Debug, Deserialize, FromRow)]
pub struct UserCreatedArtist {
    pub name: String,
    pub description: String,
    pub pictures: Option<Vec<String>>,
}
