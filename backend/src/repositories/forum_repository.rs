use serde_json::Value;
use sqlx::PgPool;

use crate::{
    Error, Result,
    models::forum::{ForumPost, ForumThread, UserCreatedForumPost, UserCreatedForumThread},
};

pub async fn create_forum_post(
    pool: &PgPool,
    forum_post: &UserCreatedForumPost,
    current_user_id: i64,
) -> Result<ForumPost> {
    let mut tx = pool.begin().await?;

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
    .fetch_one(&mut *tx)
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
    .execute(&mut *tx)
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
    .execute(&mut *tx)
    .await
    .map_err(Error::CouldNotCreateForumPost)?;

    tx.commit().await?;

    Ok(forum_post)
}

pub async fn create_forum_thread(
    pool: &PgPool,
    forum_thread: &mut UserCreatedForumThread,
    current_user_id: i64,
) -> Result<ForumThread> {
    let created_forum_thread = sqlx::query_as!(
        ForumThread,
        r#"
            INSERT INTO forum_threads (name, created_by_id, forum_sub_category_id)
            VALUES ($1, $2, $3)
            RETURNING *
        "#,
        forum_thread.name,
        current_user_id,
        forum_thread.forum_sub_category_id
    )
    .fetch_one(pool)
    .await
    .map_err(Error::CouldNotCreateForumThread)?;

    forum_thread.first_post.forum_thread_id = created_forum_thread.id;

    create_forum_post(pool, &forum_thread.first_post, current_user_id).await?;

    Ok(created_forum_thread)
}

pub async fn find_forum_overview(pool: &PgPool) -> Result<Value> {
    let forum_overview = sqlx::query!(
        r#"
        SELECT
            json_build_object(
                'forum_categories', json_agg(
                    json_build_object(
                        'id', fc.id,
                        'name', fc.name,
                        'sub_categories', (
                            SELECT
                                json_agg(
                                    json_build_object(
                                        'id', fsc.id,
                                        'name', fsc.name,
                                        'threads_amount', fsc.threads_amount,
                                        'posts_amount', fsc.posts_amount,
                                        'forbidden_classes', '[]'::jsonb,
                                        'latest_post_in_thread', CASE
                                            WHEN ft.id IS NOT NULL THEN json_build_object(
                                                'id', ft.id,
                                                'name', ft.name,
                                                'created_at', ft.latest_post_created_at,
                                                'created_by', json_build_object( -- Changed to a JSON object for user details
                                                    'id', ft.latest_post_created_by_id,
                                                    'username', ft.latest_post_created_by_username
                                                ),
                                                'posts_amount', ft.posts_amount
                                            )
                                            ELSE NULL
                                        END
                                    ) ORDER BY fsc.name
                                )
                            FROM
                                forum_sub_categories fsc
                            LEFT JOIN LATERAL (
                                SELECT
                                    ft_with_latest_post.id,
                                    ft_with_latest_post.name,
                                    ft_with_latest_post.posts_amount,
                                    fp_latest.created_at AS latest_post_created_at,
                                    fp_latest.created_by_id AS latest_post_created_by_id,
                                    u.username AS latest_post_created_by_username -- Joined to get the username
                                FROM
                                    forum_posts fp_latest
                                JOIN
                                    forum_threads ft_with_latest_post ON fp_latest.forum_thread_id = ft_with_latest_post.id
                                JOIN
                                    users u ON fp_latest.created_by_id = u.id -- Joined with the users table
                                WHERE
                                    ft_with_latest_post.forum_sub_category_id = fsc.id
                                ORDER BY
                                    fp_latest.created_at DESC
                                LIMIT 1
                            ) AS ft ON TRUE
                            WHERE
                                fsc.forum_category_id = fc.id
                        )
                    ) ORDER BY fc.name
                )
            ) AS forum_overview
        FROM
            forum_categories fc;
        "#
    )
    .fetch_one(pool)
    .await
    .expect("error getting forums");

    Ok(forum_overview.forum_overview.unwrap())
}

pub async fn find_forum_sub_category_threads(
    pool: &PgPool,
    forum_sub_category_id: i32,
) -> Result<Value> {
    let forum_sub_category = sqlx::query!(
        r#"
        SELECT
            json_strip_nulls(
                json_build_object(
                    'id', fsc.id,
                    'name', fsc.name,
                    'threads_amount', fsc.threads_amount,
                    'posts_amount', fsc.posts_amount,
                    'forbidden_classes', fsc.forbidden_classes,
                    'category', json_build_object(
                        'id', fc.id,
                        'name', fc.name
                    ),
                    'threads', (
                        SELECT
                            COALESCE(
                                json_agg(
                                    json_build_object(
                                        'id', ft.id,
                                        'name', ft.name,
                                        'created_at', ft.created_at,
                                        'posts_amount', ft.posts_amount,
                                        'latest_post', CASE
                                            WHEN fp_latest.id IS NOT NULL THEN json_build_object(
                                                'id', fp_latest.id,
                                                'created_at', fp_latest.created_at,
                                                'created_by', json_build_object(
                                                    'id', u_post.id,
                                                    'username', u_post.username
                                                )
                                            )
                                            ELSE NULL
                                        END
                                    ) ORDER BY ft.created_at DESC
                                ),
                                '[]'::json
                            )
                        FROM
                            forum_threads ft
                        LEFT JOIN LATERAL (
                            SELECT
                                fp.id,
                                fp.created_at,
                                fp.created_by_id
                            FROM
                                forum_posts fp
                            WHERE
                                fp.forum_thread_id = ft.id
                            ORDER BY
                                fp.created_at DESC
                            LIMIT 1
                        ) AS fp_latest ON TRUE
                        LEFT JOIN
                            users u_post ON fp_latest.created_by_id = u_post.id
                        WHERE
                            ft.forum_sub_category_id = fsc.id
                    )
                )
            ) AS result_json
        FROM
            forum_sub_categories fsc
        JOIN
            forum_categories fc ON fsc.forum_category_id = fc.id
        WHERE
            fsc.id = $1
        GROUP BY
            fsc.id, fc.id;
        "#,
        forum_sub_category_id
    )
    .fetch_one(pool)
    .await
    .map_err(Error::CouldNotFindForumSubCategory)?;

    //TODO: unwrap can fail, return Error::CouldNotFindForumSubCategory
    Ok(forum_sub_category.result_json.unwrap())
}

pub async fn find_forum_thread(pool: &PgPool, forum_thread_id: i64) -> Result<Value> {
    let forum_thread = sqlx::query!(
        r#"
        SELECT
            JSON_BUILD_OBJECT(
                'id', ft.id,
                'forum_sub_category_id', ft.forum_sub_category_id,
                'name', ft.name,
                'created_at', ft.created_at,
                'created_by_id', ft.created_by_id,
                'posts_amount', ft.posts_amount,
                'sticky', ft.sticky,
                'locked', ft.locked,
                'posts', JSON_AGG(
                    JSON_BUILD_OBJECT(
                        'id', fp.id,
                        'content', fp.content,
                        'created_at', fp.created_at,
                        'created_by', JSON_BUILD_OBJECT(
                            'id', u.id,
                            'username', u.username,
                            'avatar', u.avatar
                        )
                    ) ORDER BY fp.created_at ASC
                )
            ) AS thread_data
        FROM
            forum_threads AS ft
        JOIN
            forum_posts AS fp ON ft.id = fp.forum_thread_id
        JOIN
            users AS u ON fp.created_by_id = u.id
        WHERE
            ft.id = $1
        GROUP BY
            ft.id,
            ft.forum_sub_category_id,
            ft.name,
            ft.created_at,
            ft.created_by_id,
            ft.posts_amount,
            ft.sticky,
            ft.locked;
        "#,
        forum_thread_id
    )
    .fetch_one(pool)
    .await
    .map_err(Error::CouldNotFindForumThread)?;

    //TODO: unwrap can fail, return Error::CouldNotFindForumThread
    Ok(forum_thread.thread_data.unwrap())
}
