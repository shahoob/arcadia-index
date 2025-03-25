use crate::models::{
    title_group::{TitleGroup, UserCreatedTitleGroup},
    user::User,
};
use actix_web::web;
use serde_json::Value;
use sqlx::PgPool;
use std::error::Error;

pub async fn create_title_group(
    pool: &web::Data<PgPool>,
    title_group_form: &UserCreatedTitleGroup,
    current_user: &User,
) -> Result<TitleGroup, Box<dyn Error>> {
    let create_title_group_query = r#"
        INSERT INTO title_groups (master_group_id,name,name_aliases,created_by_id,description,original_language,country_from,covers,external_links,embedded_links,category,content_type,original_release_date,tags,tagline) 
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11::category_enum, $12::content_type_enum, $13, $14, $15)
        RETURNING *;
    "#;

    let created_title_group = sqlx::query_as::<_, TitleGroup>(create_title_group_query)
        .bind(&title_group_form.master_group_id)
        .bind(&title_group_form.name)
        .bind(&title_group_form.name_aliases)
        .bind(&current_user.id)
        .bind(&title_group_form.description)
        .bind(&title_group_form.original_language)
        .bind(&title_group_form.country_from)
        .bind(&title_group_form.covers)
        .bind(&title_group_form.external_links)
        .bind(&title_group_form.embedded_links)
        .bind(&title_group_form.category)
        .bind(&title_group_form.content_type)
        .bind(&title_group_form.original_release_date)
        .bind(&title_group_form.tags)
        .bind(&title_group_form.tagline)
        // .bind(&title_group_form.public_ratings)
        .fetch_one(pool.get_ref())
        .await;

    match created_title_group {
        Ok(_) => Ok(created_title_group.unwrap()),
        Err(e) => {
            println!("{:#?}", e);
            match e {
                sqlx::Error::Database(db_error) => db_error.message().to_string(),
                _ => e.to_string(),
            };
            Err(format!("could not create title group").into())
        }
    }
}

pub async fn find_title_group(
    pool: &web::Data<PgPool>,
    title_group_id: i32,
) -> Result<Value, Box<dyn Error>> {
    let title_group = sqlx::query!(r#"WITH torrent_data AS (
    SELECT 
        t.edition_group_id,
        jsonb_agg(
            to_jsonb(t) || jsonb_build_object('uploader', jsonb_build_object('id', u.id, 'username', u.username))
        ) AS torrents
    FROM torrents t
    LEFT JOIN users u ON u.id = t.created_by_id
    GROUP BY t.edition_group_id
),
edition_data AS (
    SELECT 
        eg.title_group_id,
        jsonb_agg(
            to_jsonb(eg) || jsonb_build_object('torrents', COALESCE(td.torrents, '[]'::jsonb))
        ) AS edition_groups
    FROM edition_groups eg
    LEFT JOIN torrent_data td ON td.edition_group_id = eg.id
    GROUP BY eg.title_group_id
),
artist_data AS (
    SELECT 
        aa.title_group_id,
        jsonb_agg(
            to_jsonb(aa) || jsonb_build_object('artist', to_jsonb(a))
        ) AS affiliated_artists
    FROM affiliated_artists aa
    JOIN artists a ON a.id = aa.artist_id
    GROUP BY aa.title_group_id
),
comment_data AS (
    SELECT 
        c.title_group_id,
        jsonb_agg(
            to_jsonb(c) || jsonb_build_object('created_by', jsonb_build_object('id', u.id, 'username', u.username, 'avatar', u.avatar)) 
            ORDER BY c.created_at
        ) AS title_group_comments
    FROM title_group_comments c
    LEFT JOIN users u ON u.id = c.created_by_id
    GROUP BY c.title_group_id
),
series_data AS (
    SELECT 
        tg.id AS title_group_id,
        jsonb_build_object('name', s.name, 'id', s.id) AS series
    FROM title_groups tg
    LEFT JOIN series s ON s.id = tg.series_id
)
SELECT jsonb_build_object(
    'title_group', to_jsonb(tg),
    'series', COALESCE(sd.series, '{}'::jsonb),
    'edition_groups', COALESCE(ed.edition_groups, '[]'::jsonb),
    'affiliated_artists', COALESCE(ad.affiliated_artists, '[]'::jsonb),
    'title_group_comments', COALESCE(cd.title_group_comments, '[]'::jsonb)
)
FROM title_groups tg
LEFT JOIN edition_data ed ON ed.title_group_id = tg.id
LEFT JOIN artist_data ad ON ad.title_group_id = tg.id
LEFT JOIN comment_data cd ON cd.title_group_id = tg.id
LEFT JOIN series_data sd ON sd.title_group_id = tg.id
WHERE tg.id = $1;"#, title_group_id)
        .fetch_one(pool.get_ref())
        .await;

    match title_group {
        Ok(_) => Ok(title_group.unwrap().jsonb_build_object.unwrap()),
        Err(e) => {
            println!("{:#?}", e);
            match e {
                sqlx::Error::Database(db_error) => db_error.message().to_string(),
                _ => e.to_string(),
            };
            Err(format!("could not find title group").into())
        }
    }
}
