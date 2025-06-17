use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::models::title_group::UserCreatedTitleGroup;

pub mod musicbrainz;
pub mod open_library;
pub mod tmdb;

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct ExternalDBData {
    pub title_group: UserCreatedTitleGroup,
    // pub edition_group: UserCreatedEditionGroup,
    // pub series: UserCreatedSeries
}
