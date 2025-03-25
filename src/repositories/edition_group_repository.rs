use crate::models::{
    edition_group::{EditionGroup, UserCreatedEditionGroup},
    user::User,
};
use actix_web::web;
use sqlx::PgPool;
use std::error::Error;

pub async fn create_edition_group(
    pool: &web::Data<PgPool>,
    edition_group_form: &UserCreatedEditionGroup,
    current_user: &User,
) -> Result<EditionGroup, Box<dyn Error>> {
    let create_title_group_query = r#"
        INSERT INTO edition_groups (title_group_id, name, release_date, created_by_id, description, distributor, covers, external_links, source, addtional_information) 
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9::source_enum), $10
        RETURNING *;
    "#;

    let additional_information = if edition_group_form.additional_information.is_some() {
        &edition_group_form.additional_information
    } else {
        &None
    };

    let created_edition_group = sqlx::query_as::<_, EditionGroup>(create_title_group_query)
        .bind(&edition_group_form.title_group_id)
        .bind(&edition_group_form.name)
        .bind(&edition_group_form.release_date)
        .bind(&current_user.id)
        .bind(&edition_group_form.description)
        .bind(&edition_group_form.distributor)
        .bind(&edition_group_form.covers)
        .bind(&edition_group_form.external_links)
        .bind(&edition_group_form.source)
        .bind(additional_information)
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
