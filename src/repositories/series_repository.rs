use crate::models::{
    series::{Series, UserCreatedSeries},
    user::User,
};
use serde_json::Value;
use sqlx::PgPool;
use std::error::Error;

pub async fn create_series(
    pool: &PgPool,
    series: &UserCreatedSeries,
    current_user: &User,
) -> Result<Series, Box<dyn Error>> {
    let created_series = sqlx::query_as!(
        Series,
        r#"
            INSERT INTO series (name,description,created_by_id,covers,banners,tags)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING *
        "#,
        series.name,
        series.description,
        current_user.id,
        &series.covers,
        &series.banners,
        &series.tags
    )
    .fetch_one(pool)
    .await;

    match created_series {
        Ok(_) => Ok(created_series.unwrap()),
        Err(e) => {
            println!("{:#?}", e);
            match e {
                sqlx::Error::Database(db_error) => db_error.message().to_string(),
                _ => e.to_string(),
            };
            Err(format!("could not create serie").into())
        }
    }
}

pub async fn find_series(pool: &PgPool, series_id: &i64) -> Result<Value, Box<dyn Error>> {
    let found_series = sqlx::query!(
        r#"
            WITH title_group_data AS (
                SELECT
                    tg.series_id,
                    jsonb_agg(
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
                    ) AS title_groups
                FROM title_groups tg
                WHERE tg.series_id = $1
                GROUP BY tg.series_id
            )
            SELECT jsonb_build_object(
                'series', to_jsonb(s),
                'title_groups', COALESCE(tg_data.title_groups, '[]'::jsonb)
            ) AS series_and_groups
            FROM series s
            LEFT JOIN title_group_data tg_data ON tg_data.series_id = s.id
            WHERE s.id = $1
        "#,
        series_id
    )
    .fetch_one(pool)
    .await;

    match found_series {
        Ok(series) => Ok(series.series_and_groups.unwrap()),
        Err(e) => {
            println!("{:#?}", e);
            match e {
                sqlx::Error::Database(db_error) => db_error.message().to_string(),
                _ => e.to_string(),
            };
            Err(format!("an error occured while looking for the series").into())
        }
    }
}
