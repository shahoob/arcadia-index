use crate::models::user::User;
use sqlx::{PgPool, postgres::PgQueryResult};
use std::error::Error;

pub async fn create_subscription(
    pool: &PgPool,
    item_id: &i64,
    item: &str,
    current_user: &User,
) -> Result<bool, Box<dyn Error>> {
    let result: Result<PgQueryResult, sqlx::Error>;
    match item {
        "title_group" => {
            result = sqlx::query!(
                r#"
                    INSERT INTO title_group_subscriptions (title_group_id, subscriber_id)
                    VALUES ($1, $2)
                "#,
                item_id,
                current_user.id
            )
            .execute(pool)
            .await;
        }
        _ => {
            return Err(format!("this kind of subscription is not supported").into());
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
            Err(format!("could not create serie").into())
        }
    }
}

pub async fn delete_subscription(
    pool: &PgPool,
    item_id: &i64,
    item: &str,
    current_user: &User,
) -> Result<bool, Box<dyn std::error::Error>> {
    let result: Result<PgQueryResult, sqlx::Error>;
    match item {
        "title_group" => {
            result = sqlx::query!(
                r#"
                    DELETE FROM title_group_subscriptions
                    WHERE title_group_id = $1 AND subscriber_id = $2;
                "#,
                item_id,
                current_user.id
            )
            .execute(pool)
            .await;
        }
        _ => {
            return Err(format!("this kind of subscription is not supported").into());
        }
    }

    match result {
        Ok(_) => Ok(true),
        Err(e) => {
            println!("{:#?}", e);
            Err(format!(
                "could not unsubscribe, maybe the subscription doesn't exist or isn't yours"
            )
            .into())
        }
    }
}
