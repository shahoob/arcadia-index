use sqlx::{PgPool, Postgres, Transaction};

use crate::{
    Error, Result,
    models::gift::{Gift, UserCreatedGift},
};

pub async fn create_gift(
    pool: &PgPool,
    gift: &UserCreatedGift,
    current_user_id: i64,
) -> Result<Gift> {
    let mut tx = pool.begin().await?;

    let _ = decrement_bonus_points_and_freeleech_tokens(
        &mut tx,
        current_user_id,
        gift.bonus_points,
        gift.freeleech_tokens,
    )
    .await;

    let gift = sqlx::query_as!(
        Gift,
        r#"
            INSERT INTO gifts (message, sender_id, receiver_id, bonus_points, freeleech_tokens)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING *
        "#,
        gift.message,
        current_user_id,
        gift.receiver_id,
        gift.bonus_points,
        gift.freeleech_tokens
    )
    .fetch_one(&mut *tx)
    .await
    .map_err(Error::CouldNotCreateGift)?;

    tx.commit().await?;

    Ok(gift)
}

pub async fn decrement_bonus_points_and_freeleech_tokens(
    tx: &mut Transaction<'_, Postgres>,
    current_user_id: i64,
    bonus_points: i64,
    freeleech_tokens: i32,
) -> Result<()> {
    sqlx::query!(
        r#"
           UPDATE users SET bonus_points = bonus_points - $1,
           freeleech_tokens = freeleech_tokens - $2
           WHERE id = $3
        "#,
        bonus_points,
        freeleech_tokens,
        current_user_id
    )
    .execute(&mut **tx)
    .await?;

    Ok(())
}
