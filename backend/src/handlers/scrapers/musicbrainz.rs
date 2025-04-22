use crate::{
    models::{artist::UserCreatedArtist, title_group::{create_default_title_group, ContentType, TitleGroupCategory, UserCreatedTitleGroup}}, Result
};
use actix_web::{HttpResponse, web};
use serde::Deserialize;

use musicbrainz_rs::{client::MUSICBRAINZ_CLIENT, entity::{
    artist::Artist, coverart::Coverart, release::Release, release_group::{ReleaseGroup, ReleaseGroupPrimaryType}
}};
use musicbrainz_rs::prelude::*;

// TODO: Make this configurable incase user wants different musicbrainz instance
const MUSICBRAINZ_WEB_SERVICE_URL: &str = "https://musicbrainz.org";

#[derive(Debug, Deserialize)]
struct MusicBrainzQuery {
    id: String,
}

pub async fn get_musicbrainz_release_group_info(query: web::Query<MusicBrainzQuery>) -> Result<HttpResponse> {
    let release_group = ReleaseGroup::fetch()
    .id(&query.id)
    .execute().await.expect("Couldn't fetch release group");

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
