use crate::{
    Arcadia, Result,
    handlers::UserId,
    models::artist::{
        AffiliatedArtist, Artist, ArtistAndTitleGroupsLite, ArtistLite,
        UserCreatedAffiliatedArtist, UserCreatedArtist,
    },
    repositories::artist_repository::{
        create_artists, create_artists_affiliation, find_artist_publications, find_artists_lite,
    },
};
use actix_web::{HttpResponse, web};
use serde::Deserialize;
use utoipa::{IntoParams, ToSchema};

#[utoipa::path(
    post,
    path = "/api/artists",
    responses(
        (status = 200, description = "Successfully created the artists, returned in the same order as the one sent", body=Vec<Artist>),
    )
)]
pub async fn add_artists(
    artists: web::Json<Vec<UserCreatedArtist>>,
    arc: web::Data<Arcadia>,
    current_user_id: UserId,
) -> Result<HttpResponse> {
    let artists = create_artists(&arc.pool, &artists, current_user_id.0).await?;

    Ok(HttpResponse::Created().json(artists))
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
    current_user_id: UserId,
) -> Result<HttpResponse> {
    let affiliations = create_artists_affiliation(&arc.pool, &artists, current_user_id.0).await?;

    Ok(HttpResponse::Created().json(affiliations))
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
        (status = 200, description = "Successfully got the artist's pulications", body=ArtistAndTitleGroupsLite),
    )
)]
pub async fn get_artist_publications(
    query: web::Query<GetArtistPublicationsQuery>,
    arc: web::Data<Arcadia>,
) -> Result<HttpResponse> {
    let artist_publication = find_artist_publications(&arc.pool, &query.id).await?;

    Ok(HttpResponse::Ok().json(artist_publication))
}

#[derive(Debug, Deserialize, ToSchema, IntoParams)]
pub struct GetArtistLiteQuery {
    name: String,
}

#[utoipa::path(
    get,
    path = "/api/search/artist/lite",
    params (GetArtistLiteQuery),
    description = "Case insensitive",
    responses(
        (status = 200, description = "Successfully got the artists and some data about them", body=Vec<ArtistLite>),
    )
)]
pub async fn get_artists_lite(
    query: web::Query<GetArtistLiteQuery>,
    arc: web::Data<Arcadia>,
) -> Result<HttpResponse> {
    let artists = find_artists_lite(&arc.pool, &query.name).await?;

    Ok(HttpResponse::Ok().json(artists))
}
