use crate::{
    Error, Result,
    models::{
        title_group::{TitleGroup, UserCreatedTitleGroup},
        user::User,
    },
};
use serde_json::Value;
use sqlx::PgPool;

pub async fn create_title_group(
    pool: &PgPool,
    title_group_form: &UserCreatedTitleGroup,
    current_user: &User,
) -> Result<TitleGroup> {
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
        .fetch_one(pool)
        .await
        .map_err(Error::CouldNotCreateTitleGroup)?;

    Ok(created_title_group)
}

pub async fn find_title_group(
    pool: &PgPool,
    title_group_id: i64,
    current_user: &User,
) -> Result<Value> {
    let title_group = sqlx::query!(r#"WITH torrent_data AS (
    SELECT 
        t.edition_group_id,
        jsonb_agg(
            to_jsonb(t) || jsonb_build_object('uploader', jsonb_build_object('id', u.id, 'username', u.username))
            ORDER BY t.size DESC
        ) AS torrents
    FROM torrents_and_reports t
    LEFT JOIN users u ON u.id = t.created_by_id
    GROUP BY t.edition_group_id
),
torrent_request_data AS (
    SELECT 
        tr.title_group_id,
        jsonb_agg(to_jsonb(tr)) AS torrent_requests
    FROM torrent_requests tr
    LEFT JOIN users u ON u.id = tr.created_by_id
    GROUP BY tr.title_group_id
),
edition_data AS (
    SELECT 
        eg.title_group_id,
        jsonb_agg(
            to_jsonb(eg) || jsonb_build_object('torrents', COALESCE(td.torrents, '[]'::jsonb))
            ORDER BY eg.release_date
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
),
subscription_data AS (
    SELECT 
        id, 
        EXISTS(
            SELECT 1 
            FROM title_group_subscriptions tgs 
            WHERE tgs.title_group_id = tg.id 
              AND tgs.subscriber_id = $1
        ) AS is_subscribed
    FROM title_groups tg
)
SELECT 
    to_jsonb(tg) || jsonb_build_object(
    'series', COALESCE(sd.series, '{}'::jsonb),
    'edition_groups', COALESCE(ed.edition_groups, '[]'::jsonb),
    'affiliated_artists', COALESCE(ad.affiliated_artists, '[]'::jsonb),
    'title_group_comments', COALESCE(cd.title_group_comments, '[]'::jsonb),
    'torrent_requests', COALESCE(trd.torrent_requests, '[]'::jsonb),
    'is_subscribed', COALESCE(sud.is_subscribed, false)
) AS title_group_data
FROM title_groups tg
LEFT JOIN edition_data ed ON ed.title_group_id = tg.id
LEFT JOIN artist_data ad ON ad.title_group_id = tg.id
LEFT JOIN comment_data cd ON cd.title_group_id = tg.id
LEFT JOIN series_data sd ON sd.title_group_id = tg.id
LEFT JOIN torrent_request_data trd ON trd.title_group_id = tg.id
LEFT JOIN subscription_data sud ON sud.id = tg.id
WHERE tg.id = $2;"#, current_user.id, title_group_id)
        .fetch_one(pool)
        .await?;

    Ok(title_group.title_group_data.unwrap())
}
pub async fn find_lite_title_group_info(pool: &PgPool, title_group_id: i64) -> Result<Value> {
    let title_group = sqlx::query!(
        r#"SELECT jsonb_build_object(
    'id', tg.id, 'content_type', tg.content_type, 'name', tg.name,
    'edition_groups', COALESCE(
        jsonb_agg(
            jsonb_build_object(
                'id', eg.id,
                'name', eg.name,
                'release_date', eg.release_date,
                'distributor', eg.distributor,
                'source', eg.source,
                'additional_information', eg.additional_information
            )
        ) FILTER (WHERE eg.id IS NOT NULL), 
        '[]'::jsonb
    )
)
FROM title_groups tg
LEFT JOIN edition_groups eg ON eg.title_group_id = tg.id
WHERE tg.id = $1
GROUP BY tg.id;"#,
        title_group_id
    )
    .fetch_one(pool)
    .await?;

    Ok(title_group.jsonb_build_object.unwrap())
}
