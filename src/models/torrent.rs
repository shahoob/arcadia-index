use std::collections::HashMap;

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Torrent {
    pub id: i64,
    pub edition_group: i64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub created_by: i32,
    pub release_name: Option<String>,
    pub release_group: String,
    pub description: Option<String>, // specific to the torrent
    pub file_amount_per_type: HashMap<String, i16>, // (5 mp3, 1 log, 5 jpg, etc.)
    pub uploaded_as_anonymous: bool,
    pub file_list: Vec<String>,
    pub mediainfo: String,
    pub trumpable: Option<String>, // description of why it is trumpable
    pub staff_checked: bool,
    pub size: i64, // in bytes
}
