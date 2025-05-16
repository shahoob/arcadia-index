use rand::{
    distr::{Alphanumeric, SampleString},
    rng,
};
use sqlx::{PgPool, Postgres, Transaction};

use crate::{
    Error, Result,
    models::invitation::{Invitation, SentInvitation},
};

pub async fn create_invitation(
    pool: &PgPool,
    invitation: &SentInvitation,
    current_user_id: i64,
) -> Result<Invitation> {
    let invitation_key: String = Alphanumeric.sample_string(&mut rng(), 50);

    let mut tx = pool.begin().await?;

    let _ = decrement_invitations_available(&mut tx, current_user_id).await;

    // TODO: make invitation expiration configurable
    let sent_invitation = sqlx::query_as!(
        Invitation,
        r#"
            INSERT INTO invitations (message, invitation_key, sender_id, receiver_email, expires_at)
            VALUES ($1, $2, $3, $4, NOW() + INTERVAL '3 days')
            RETURNING *
        "#,
        invitation.message,
        invitation_key,
        current_user_id,
        invitation.receiver_email
    )
    .fetch_one(pool)
    .await
    .map_err(Error::CouldNotCreateInvitation)?;

    tx.commit().await?;

    Ok(sent_invitation)
}

pub async fn does_unexpired_invitation_exist(
    pool: &PgPool,
    invitation_key: &str,
) -> Result<Invitation> {
    let invitation = sqlx::query_as!(
        Invitation,
        r#"
           SELECT * FROM invitations
           WHERE invitation_key = $1
           AND expires_at > NOW()
        "#,
        invitation_key
    )
    .fetch_one(pool)
    .await
    .map_err(|_| Error::InvitationKeyInvalid)?;

    Ok(invitation)
}

pub async fn decrement_invitations_available(
    tx: &mut Transaction<'_, Postgres>,
    current_user_id: i64,
) -> Result<()> {
    sqlx::query!(
        r#"
           UPDATE users SET invitations = invitations - 1
           WHERE id = $1
        "#,
        current_user_id
    )
    .execute(&mut **tx)
    .await?;

    Ok(())
}
