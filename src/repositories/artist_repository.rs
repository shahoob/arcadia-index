use std::error::Error;

use actix_web::web;

use sqlx::PgPool;

use crate::models::{
    artist::{Artist, UserCreatedArtist},
    edition_group::{EditionGroup, UserCreatedEditionGroup},
    title_group::{TitleGroup, UserCreatedTitleGroup},
    user::User,
};

pub async fn create_artist(
    pool: &web::Data<PgPool>,
    artist: &UserCreatedArtist,
    current_user: &User,
) -> Result<Artist, Box<dyn Error>> {
    let create_artist_query = r#"
        INSERT INTO artists (name, description, pictures, created_by) 
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
