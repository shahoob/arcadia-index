use sqlx::PgPool;

use crate::{
    Error, Result,
    models::forum::{ForumPost, UserCreatedForumPost},
};

pub async fn create_forum_post(
    pool: &PgPool,
    forum_post: &UserCreatedForumPost,
    current_user_id: i64,
) -> Result<ForumPost> {
    let forum_post = sqlx::query_as!(
        ForumPost,
        r#"
            INSERT INTO forum_posts (content, created_by_id, forum_thread_id)
            VALUES ($1, $2, $3)
            RETURNING *
        "#,
        forum_post.content,
        current_user_id,
        forum_post.forum_thread_id
    )
    .fetch_one(pool)
    .await
    .map_err(Error::CouldNotCreateForumPost)?;

    sqlx::query!(
        r#"
        UPDATE forum_threads
        SET posts_amount = posts_amount + 1
        WHERE id = $1;
        "#,
        forum_post.forum_thread_id
    )
    .execute(pool)
    .await
    .map_err(Error::CouldNotCreateForumPost)?;

    sqlx::query!(
        r#"
        UPDATE forum_sub_categories
        SET posts_amount = posts_amount + 1
        WHERE id = (SELECT forum_sub_category_id FROM forum_threads WHERE id = $1);
        "#,
        forum_post.forum_thread_id
    )
    .execute(pool)
    .await
    .map_err(Error::CouldNotCreateForumPost)?;
    Ok(forum_post)
}
