use crate::{
    models::{artist::UserCreatedArtist, title_group::{create_default_title_group, ContentType, TitleGroupCategory, UserCreatedTitleGroup}}, Result
};
use actix_web::{HttpResponse, web};
use serde::Deserialize;

use musicbrainz_rs::entity::{
    artist::Artist, coverart::Coverart, release::Release, release_group::{ReleaseGroup, ReleaseGroupPrimaryType}
};
use musicbrainz_rs::prelude::*;

#[derive(Debug, Deserialize)]
struct MusicBrainzQuery {
    id: String,
}

impl TryFrom<ReleaseGroupPrimaryType> for TitleGroupCategory {
    fn try_from(value: ReleaseGroupPrimaryType) -> Result<Self> {
        match value {
            ReleaseGroupPrimaryType::Album => Ok(Self::Album),
            ReleaseGroupPrimaryType::Ep => Ok(Self::Ep),
            ReleaseGroupPrimaryType::Single => Ok(Self::Single),
            ReleaseGroupPrimaryType::Broadcast => todo!(),
            ReleaseGroupPrimaryType::Other => Ok(Self::Other),
        }
    }
}

pub async fn get_musicbrainz_release_group_info(query: web::Query<MusicBrainzQuery>) -> Result<HttpResponse> {
    let release_group = ReleaseGroup::fetch()
    .id(&query.id)
    .execute().await?;

    let primary_type = release_group.primary_type;

    let title_group = UserCreatedTitleGroup {
        name: release_group.title,

        external_links: vec![todo!("will put musicbrainz release group url and other external links found in it")],

        content_type: ContentType::Music,
        category: if primary_type.is_some() { TitleGroupCategory::try_from(primary_type.unwrap()) } else { Some(TitleGroupCategory::Other) },
        ..create_default_title_group()
    };

    todo!("Not enough info is programmed to be retrived yet from MusicBrainz")
}
