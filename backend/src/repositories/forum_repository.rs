use serde_json::Value;
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
