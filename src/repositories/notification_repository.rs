use crate::{Error, Result};
use sqlx::PgPool;

pub async fn notify_users(
    pool: &PgPool,
    event: &str,
    item_id: &i64,
    title: &str,
    message: &str,
) -> Result<()> {
    match event {
        "torrent_uploaded" => {
            sqlx::query!(
                r#"
                    WITH subscriber_ids AS (
                        SELECT subscriber_id AS user_id
                        FROM title_group_subscriptions
                        WHERE title_group_id = $1
                    )
                    INSERT INTO notifications (receiver, title, message, notification_type, item_id)
                    SELECT
                        user_id,
                        $2,
                        $3,
                        'TitleGroup'::notification_item_enum,
                        $1
                    FROM subscriber_ids
                "#,
                item_id,
                title,
                message
            )
            .execute(pool)
            .await
            .map_err(Error::CouldNotCreateNotification)?;
        }
        _ => {
            return Err(Error::UnsupportedNotification(event.into()));
        }
    }

    Ok(())
}
