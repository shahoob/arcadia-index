use crate::{
    models::{artist::UserCreatedArtist, title_group::{create_default_title_group, ContentType, TitleGroupCategory, UserCreatedTitleGroup}}, Result
};
use actix_web::{HttpResponse, web};
use serde::Deserialize;

use musicbrainz_rs::{ entity::{
    artist::Artist, coverart::Coverart, release::Release, release_group::{ReleaseGroup, ReleaseGroupPrimaryType}
}, Error as MBError};
use musicbrainz_rs::prelude::*;

// TODO: Make this configurable incase user wants different musicbrainz instance
const MUSICBRAINZ_WEB_SERVICE_URL: &str = "https://musicbrainz.org";

#[derive(Debug, Deserialize)]
struct MusicBrainzQuery {
    id: String,
}

impl actix_web::ResponseError for MBError {
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        match self {
            Error::MusicbrainzError(musicbrainz_error) => todo!(),
            Error::NotFound(query) => actix_web::HttpResponse::build(actix_web::http::StatusCode::NOT_FOUND).json(serde_json::json!({
                "error": format!("MusicBrainz could not find your requested item for query: {}", query),
            })),
            _ => actix_web::HttpResponse::build(actix_web::http::StatusCode::I).json(serde_json::json!({
                "error": format!("MusicBrainz could not find your requested item for query: {}", query),
            }))
        }
    }
}

pub async fn get_musicbrainz_release_group_info(query: web::Query<MusicBrainzQuery>) -> Result<HttpResponse> {
    let release_group = ReleaseGroup::fetch()
    .id(&query.id)
    .execute().await?;

    let mut urls: Vec<String> = vec![format!("{}/release_group/{}", MUSICBRAINZ_WEB_SERVICE_URL, query.id)];

    // release_group.relations

    let title_group = UserCreatedTitleGroup {
        name: release_group.title,

        external_links: urls,

        content_type: ContentType::Music,
        category: todo!(),
        ..create_default_title_group()
    };

    
    todo!("Not enough info is programmed to be retrived yet from MusicBrainz");

    // Ok(HttpResponse::Ok().json(serde_json::json!({"title_group": title_group})));
}
