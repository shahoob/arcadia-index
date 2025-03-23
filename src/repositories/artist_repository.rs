use crate::models::{
    artist::{Artist, UserCreatedArtist},
    title_group::{AffiliatedArtist, UserCreatedAffiliatedArtist},
    user::User,
};
use actix_web::web;
use sqlx::PgPool;
use std::error::Error;

pub async fn create_artist(
    pool: &web::Data<PgPool>,
    artist: &UserCreatedArtist,
    current_user: &User,
) -> Result<Artist, Box<dyn Error>> {
    let create_artist_query = r#"
        INSERT INTO artists (name, description, pictures, created_by_id) 
        VALUES ($1, $2, $3, $4)
        RETURNING *;
    "#;

    let created_artist = sqlx::query_as::<_, Artist>(create_artist_query)
        .bind(&artist.name)
        .bind(&artist.description)
        .bind(&artist.pictures)
        .bind(&current_user.id)
        .fetch_one(pool.get_ref())
        .await;

    match created_artist {
        Ok(_) => Ok(created_artist.unwrap()),
        Err(e) => {
            println!("{:#?}", e);
            match e {
                sqlx::Error::Database(db_error) => db_error.message().to_string(),
                _ => e.to_string(),
            };
            Err(format!("could not create artist").into())
        }
    }
}

pub async fn create_artists_affiliation(
    pool: &web::Data<PgPool>,
    artists: &Vec<UserCreatedAffiliatedArtist>,
    current_user: &User,
) -> Result<Vec<AffiliatedArtist>, Box<dyn Error>> {
    let values: Vec<String> = (0..artists.len())
        .map(|i| {
            format!(
                "(${}, ${}, ${}, ${}, ${})",
                i * 5 + 1,
                i * 5 + 2,
                i * 5 + 3,
                i * 5 + 4,
                i * 5 + 5
            )
        })
        .collect();

    let query = format!(
        "INSERT INTO affiliated_artists (title_group_id, artist_id, status, nickname, created_by_id) VALUES {} RETURNING *",
        values.join(", ")
    );

    let mut q = sqlx::query_as::<_, AffiliatedArtist>(&query);
    for artist in artists {
        q = q
            .bind(artist.title_group_id)
            .bind(artist.artist_id)
            .bind(artist.status.clone())
            .bind(artist.nickname.clone())
            .bind(current_user.id);
    }

    match q.fetch_all(pool.as_ref()).await {
        Ok(affiliated_artists) => Ok(affiliated_artists),
        Err(e) => {
            println!("{:#?}", e);
            let error_message = match e {
                sqlx::Error::Database(db_error) => db_error.message().to_string(),
                _ => e.to_string(),
            };
            Err(format!("could not create artist: {}", error_message).into()) // Return the error properly
        }
    }

    // match affiliated_artists {
    //     Err(e) => {}
    // }
}
