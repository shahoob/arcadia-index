use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use utoipa::ToSchema;

use crate::models::{
    artist::ArtistLite, entity::EntityLite, master_group::MasterGroupLite,
    title_group::TitleGroupHierarchyLite,
};

#[derive(Debug, Serialize, Deserialize, ToSchema, sqlx::Type)]
#[sqlx(type_name = "collage_category_enum")]
pub enum CollageCategory {
    Personal,
    #[sqlx(rename = "Staff Picks")]
    #[serde(rename = "Staff Picks")]
    StaffPicks,
    External, // replicates a collage from somewhere else (yt playlist, magazine's picks, etc.)
    Theme,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, sqlx::Type)]
#[sqlx(type_name = "collage_type_enum")]
pub enum CollageType {
    Artist,
    Entity,
    TitleGroup,
    MasterGroup,
}

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Collage {
    pub id: i64,
    #[schema(value_type = String, format = DateTime)]
    pub created_at: DateTime<Local>,
    pub created_by_id: i64,
    pub name: String,
    pub covers: String,
    pub description: String,
    pub tags: Vec<String>,
    pub category: CollageCategory,
    pub collage_type: CollageType,
}

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct UserCreatedCollage {
    pub name: String,
    pub covers: String,
    pub description: String,
    pub tags: Vec<String>,
    pub category: CollageCategory,
    pub collage_type: CollageType,
}

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct CollageEntry {
    pub id: i64,
    #[schema(value_type = String, format = DateTime)]
    pub created_at: DateTime<Local>,
    pub created_by_id: i64,
    pub artist_id: Option<i64>,
    pub entity_id: Option<i64>,
    pub title_group_id: Option<i64>,
    pub master_group_id: Option<i64>,
    pub collage_id: i64,
    pub note: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct UserCreatedCollageEntry {
    pub artist_id: Option<i64>,
    pub entity_id: Option<i64>,
    pub title_group_id: Option<i64>,
    pub master_group_id: Option<i64>,
    pub collage_id: i64,
    pub note: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct CollageEntryHierarchy {
    pub id: i64,
    #[schema(value_type = String, format = DateTime)]
    pub created_at: DateTime<Local>,
    pub created_by_id: i64,
    pub artist_id: Option<i64>,
    pub artist: Option<ArtistLite>,
    pub entity_id: Option<i64>,
    pub entity: Option<EntityLite>,
    pub title_group_id: Option<i64>,
    pub title_group: Option<TitleGroupHierarchyLite>,
    pub master_group_id: Option<i64>,
    pub master_group: Option<MasterGroupLite>,
    pub collage_id: i64,
    pub note: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CollageAndAssociatedData {
    pub collage: Collage,
    pub entries: Vec<CollageEntryHierarchy>,
}
