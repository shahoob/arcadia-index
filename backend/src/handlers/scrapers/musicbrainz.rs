use actix_web::{HttpResponse, web};
use chrono::{DateTime, NaiveDate, TimeZone, Utc};
use musicbrainz_rs::{
    Fetch, FetchCoverart,
    client::MusicBrainzClient,
    entity::{
        CoverartResponse,
        release_group::{ReleaseGroup, ReleaseGroupPrimaryType},
    },
};
use serde::Deserialize;

use crate::{
    Arcadia, Error, Result,
    handlers::scrapers::ExternalDBData,
    models::title_group::{ContentType, UserCreatedTitleGroup, create_default_title_group},
};

fn naive_date_to_utc_midnight(date: NaiveDate) -> DateTime<Utc> {
    Utc.from_local_datetime(&date.and_hms_opt(0, 0, 0).unwrap())
        .unwrap()
}

#[derive(Debug, Deserialize)]
pub struct GetMusicbrainzQuery {
    id: String,
}

#[utoipa::path(
    post,
    path = "/api/external_db/musicbrainz",
    responses(
        (status = 200, description = "", body=ExternalDBData),
    )
)]
pub async fn get_musibrainz_data(
    query: web::Query<GetMusicbrainzQuery>,
    arc: web::Data<Arcadia>,
) -> Result<HttpResponse> {
    let mut client = MusicBrainzClient::default();
    client
        .set_user_agent(&format!("{} ({})", arc.tracker_name, arc.frontend_url))
        .unwrap();
    let musicbrainz_title_group = ReleaseGroup::fetch()
        .id(&query.id)
        .execute_with_client(&client)
        .await
        .map_err(Error::ErrorGetMusicbrainzReleaseGroup)?;
    let cover = ReleaseGroup::fetch_coverart()
        .id(&query.id)
        .execute_with_client(&client)
        .await
        .map_or_else(
            |_| String::new(),
            |c| match c {
                CoverartResponse::Url(url) => url,
                CoverartResponse::Json(coverart) => coverart
                    .images
                    .first()
                    .map_or_else(String::new, |img| img.image.clone()),
            },
        );

    let title_group = UserCreatedTitleGroup {
        name: musicbrainz_title_group.title,
        original_release_date: musicbrainz_title_group
            .first_release_date
            .map(naive_date_to_utc_midnight)
            .unwrap_or_else(|| {
                naive_date_to_utc_midnight(NaiveDate::from_ymd_opt(1900, 1, 1).unwrap())
            }),
        category: Some(
            musicbrainz_title_group
                .primary_type
                .unwrap_or(ReleaseGroupPrimaryType::UnrecognizedReleaseGroupPrimaryType)
                .into(),
        ),
        content_type: ContentType::Music,
        external_links: vec![format!(
            "https://musicbrainz.org/release-group/{}",
            query.id
        )],
        covers: vec![cover],
        ..create_default_title_group()
    };

    Ok(HttpResponse::Ok().json(serde_json::json!({"title_group": title_group})))
}
