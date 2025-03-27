use crate::models::{
    artist::{Artist, UserCreatedArtist},
    title_group::{AffiliatedArtist, UserCreatedAffiliatedArtist},
    user::User,
};
use actix_web::web;
use serde_json::Value;
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

pub async fn find_artist_publications(
    pool: &web::Data<PgPool>,
    artist_id: &i32,
) -> Result<Value, Box<dyn Error>> {
    // TODO: only select the required info about the torrents (mediainfo etc is not necessary)
    let artist_publications = sqlx::query!(
        r#"WITH artist_title_groups AS (
    SELECT DISTINCT tg.*
    FROM title_groups tg
    JOIN affiliated_artists aa ON aa.title_group_id = tg.id
    WHERE aa.artist_id = $1
)
SELECT json_agg(
    to_jsonb(tg) || jsonb_build_object(
        'edition_groups', (
            SELECT COALESCE(jsonb_agg(
                to_jsonb(eg) || jsonb_build_object(
                    'torrents', (
                        SELECT COALESCE(jsonb_agg(to_jsonb(t)), '[]'::jsonb)
                        FROM torrents t
                        WHERE t.edition_group_id = eg.id
                    )
                )
            ), '[]'::jsonb)
            FROM edition_groups eg
            WHERE eg.title_group_id = tg.id
        )
    )
) AS artist_content
FROM artist_title_groups tg;"#,
        artist_id
    )
    .fetch_one(pool.get_ref())
    .await;

    match artist_publications {
        Ok(_) => Ok(artist_publications.unwrap().artist_content.unwrap()),
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
