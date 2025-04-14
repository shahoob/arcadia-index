use crate::{
    Arcadia, Result,
    models::{
        artist::{Artist, UserCreatedArtist},
        title_group::{AffiliatedArtist, UserCreatedAffiliatedArtist},
        user::User,
    },
    repositories::artist_repository::{
        create_artist, create_artists_affiliation, find_artist_publications,
    },
};
use actix_web::{HttpResponse, web};
use serde::Deserialize;
use utoipa::{IntoParams, ToSchema};

#[utoipa::path(
    post,
    path = "/api/artist",
    responses(
        (status = 200, description = "Successfully created the artist", body=Artist),
    )
)]
pub async fn add_artist(
    artist: web::Json<UserCreatedArtist>,
    arc: web::Data<Arcadia>,
    current_user: User,
) -> Result<HttpResponse> {
    let artist = create_artist(&arc.pool, &artist, &current_user).await?;

    Ok(HttpResponse::Created().json(artist))
}

#[utoipa::path(
    post,
    path = "/api/affiliated-artists",
    responses(
        (status = 200, description = "Successfully created the artist affiliations", body=Vec<AffiliatedArtist>),
    )
)]
pub async fn add_affiliated_artists(
    artists: web::Json<Vec<UserCreatedAffiliatedArtist>>,
    arc: web::Data<Arcadia>,
    current_user: User,
) -> Result<HttpResponse> {
    let affiliation = create_artists_affiliation(&arc.pool, &artists, &current_user).await?;

    Ok(HttpResponse::Created().json(affiliation))
}

#[derive(Debug, Deserialize, ToSchema, IntoParams)]
pub struct GetArtistPublicationsQuery {
    id: i64,
}

#[utoipa::path(
    get,
    path = "/api/artist",
    params (GetArtistPublicationsQuery),
    responses(
        (status = 200, description = "Successfully got the artist's pulications"),
    )
)]
pub async fn get_artist_publications(
    query: web::Query<GetArtistPublicationsQuery>,
    arc: web::Data<Arcadia>,
) -> Result<HttpResponse> {
    let artist_publication = find_artist_publications(&arc.pool, &query.id).await?;

    Ok(HttpResponse::Created().json(artist_publication))
}
