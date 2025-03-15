use std::error::Error;

use actix_web::web;

use sqlx::PgPool;

use crate::models::{
    title_group::{TitleGroup, UserCreatedTitleGroup},
    user::User,
};

pub async fn create_title_group(
    pool: &web::Data<PgPool>,
    title_group_form: &UserCreatedTitleGroup,
    current_user: &User,
) -> Result<TitleGroup, Box<dyn Error>> {
    let create_title_group_query = r#"
        INSERT INTO title_groups (master_group,name,name_aliases,created_by,description,original_language,country_from,covers,external_links,embedded_links,category,public_ratings) 
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
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
        .bind(&title_group_form.public_ratings)
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
