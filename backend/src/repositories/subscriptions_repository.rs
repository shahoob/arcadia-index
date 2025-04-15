use crate::{Error, Result, models::user::User};
use sqlx::PgPool;

pub async fn create_subscription(
    pool: &PgPool,
    item_id: &i64,
    item: &str,
    current_user: &User,
) -> Result<()> {
    match item {
        "title_group" => {
            sqlx::query!(
                r#"
                    INSERT INTO title_group_subscriptions (title_group_id, subscriber_id)
                    VALUES ($1, $2)
                "#,
                item_id,
                current_user.id
            )
            .execute(pool)
            .await
            .map_err(Error::CouldNotCreateTitleGroupSubscription)?;

            Ok(())
        }
        _ => Err(Error::UnsupportedSubscription(item.into())),
    }
}

pub async fn delete_subscription(
    pool: &PgPool,
    item_id: &i64,
    item: &str,
    current_user: &User,
) -> Result<()> {
    match item {
        "title_group" => {
            let _ = sqlx::query!(
                r#"
                    DELETE FROM title_group_subscriptions
                    WHERE title_group_id = $1 AND subscriber_id = $2;
                "#,
                item_id,
                current_user.id
            )
            .execute(pool)
            .await?;

            // TODO: check result.rows_affected()
            Ok(())
        }
        _ => Err(Error::UnsupportedSubscription(item.into())),
    }
}
