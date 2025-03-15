use std::error::Error;

use actix_web::web;

use sqlx::PgPool;

use crate::models::{
    edition_group::{EditionGroup, UserCreatedEditionGroup},
    title_group::{TitleGroup, UserCreatedTitleGroup},
    user::User,
};

pub async fn create_edition_group(
    pool: &web::Data<PgPool>,
    edition_group_form: &UserCreatedEditionGroup,
    current_user: &User,
) -> Result<EditionGroup, Box<dyn Error>> {
    let create_title_group_query = r#"
        INSERT INTO edition_groups (title_group, name, release_date, created_by, description, distributor, covers, external_links, language, source) 
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
        RETURNING *;
    "#;

    let created_edition_group = sqlx::query_as::<_, EditionGroup>(create_title_group_query)
        .bind(&edition_group_form.title_group_id)
        .bind(&edition_group_form.name)
        .bind(&edition_group_form.release_date)
        .bind(&current_user.id)
        .bind(&edition_group_form.description)
        .bind(&edition_group_form.distributor)
        .bind(&edition_group_form.covers)
        .bind(&edition_group_form.external_links)
        .bind(&edition_group_form.language)
        .bind(&edition_group_form.source)
        .fetch_one(pool.get_ref())
        .await;

    match created_edition_group {
        Ok(_) => Ok(created_edition_group.unwrap()),
        Err(e) => {
            println!("{:#?}", e);
            match e {
                sqlx::Error::Database(db_error) => db_error.message().to_string(),
                _ => e.to_string(),
            };
            Err(format!("could not create edition group").into())
        }
    }
}
