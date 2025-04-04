use crate::{
    Arcadia, Result,
    models::{artist::UserCreatedArtist, title_group::UserCreatedAffiliatedArtist, user::User},
    repositories::artist_repository::{
        create_artist, create_artists_affiliation, find_artist_publications,
    },
};
use actix_web::{HttpResponse, web};
use serde::Deserialize;

pub async fn add_artist(
    artist: web::Json<UserCreatedArtist>,
    arc: web::Data<Arcadia>,
    current_user: User,
) -> Result<HttpResponse> {
    let artist = create_artist(&arc.pool, &artist, &current_user).await?;

    Ok(HttpResponse::Created().json(artist))
}

pub async fn add_affiliated_artists(
    artists: web::Json<Vec<UserCreatedAffiliatedArtist>>,
    arc: web::Data<Arcadia>,
    current_user: User,
) -> Result<HttpResponse> {
    let affiliation = create_artists_affiliation(&arc.pool, &artists, &current_user).await?;

    Ok(HttpResponse::Created().json(affiliation))
}

#[derive(Debug, Deserialize)]
pub struct GetArtistPublicationsQuery {
    id: i64,
}

pub async fn get_artist_publications(
    query: web::Query<GetArtistPublicationsQuery>,
    arc: web::Data<Arcadia>,
) -> Result<HttpResponse> {
    let artist_publication = find_artist_publications(&arc.pool, &query.id).await?;

    Ok(HttpResponse::Created().json(artist_publication))
}
