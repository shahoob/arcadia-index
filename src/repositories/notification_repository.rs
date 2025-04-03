use actix_web::web;
use sqlx::{PgPool, postgres::PgQueryResult};
use std::error::Error;

pub async fn notify_users(
    pool: &web::Data<PgPool>,
    event: &str,
    item_id: &i64,
    title: &str,
    message: &str,
) -> Result<bool, Box<dyn Error>> {
    let result: Result<PgQueryResult, sqlx::Error>;
    match event {
        "torrent_uploaded" => {
            result = sqlx::query!(
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
            .execute(pool.get_ref())
            .await;
        }
        _ => {
            return Err(format!("this kind of notification is not supported").into());
        }
    }

    match result {
        Ok(_) => Ok(true),
        Err(e) => {
            println!("{:#?}", e);
            match e {
                sqlx::Error::Database(db_error) => db_error.message().to_string(),
                _ => e.to_string(),
            };
            Err(format!("could not notify users").into())
        }
    }
}
