use crate::models::{
    title_group_comment::{TitleGroupComment, UserCreatedTitleGroupComment},
    user::User,
};
use actix_web::web;
use sqlx::PgPool;
use std::error::Error;

pub async fn create_title_group_comment(
    pool: &web::Data<PgPool>,
    title_group_comment: &UserCreatedTitleGroupComment,
    current_user: &User,
) -> Result<TitleGroupComment, Box<dyn Error>> {
    let create_title_group_comment_query = r#"
        INSERT INTO title_group_comments (content,title_group_id,created_by_id,refers_to_torrent_id,answers_to_comment_id) 
        VALUES ($1, $2, $3, $4, $5)
        RETURNING *;
    "#;

    let created_title_group_comment =
        sqlx::query_as::<_, TitleGroupComment>(create_title_group_comment_query)
            .bind(&title_group_comment.content)
            .bind(&title_group_comment.title_group_id)
            .bind(&current_user.id)
            .bind(&title_group_comment.refers_to_torrent_id)
            .bind(&title_group_comment.answers_to_comment_id)
            .fetch_one(pool.get_ref())
            .await;

    match created_title_group_comment {
        Ok(_) => Ok(created_title_group_comment.unwrap()),
        Err(e) => {
            println!("{:#?}", e);
            match e {
                sqlx::Error::Database(db_error) => db_error.message().to_string(),
                _ => e.to_string(),
            };
            Err(format!("could not create title group comment").into())
        }
    }
}
