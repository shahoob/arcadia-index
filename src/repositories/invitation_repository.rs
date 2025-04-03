use std::error::Error;

use rand::{
    distr::{Alphanumeric, SampleString},
    rng,
};
use sqlx::PgPool;

use crate::models::{
    invitation::{Invitation, SentInvitation},
    user::User,
};

pub async fn create_invitation(
    pool: &PgPool,
    invitation: &SentInvitation,
    current_user: &User,
) -> Result<Invitation, Box<dyn Error>> {
    let invitation_key: String = Alphanumeric.sample_string(&mut rng(), 50);

    // TODO: make this properly atomic with a db transaction
    match set_invitations_available(pool, current_user.invitations - 1, current_user).await {
        Ok(_) => {}
        Err(_) => {
            return Err(format!("could not remove invite from counter").into());
        }
    }

    // TODO: make invitation expiration configurable
    let sent_invitation = sqlx::query_as!(
        Invitation,
        r#"
            INSERT INTO invitations (message, invitation_key, sender_id, receiver_email, expires_at)
            VALUES ($1, $2, $3, $4, CURRENT_TIMESTAMP + INTERVAL '3 days')
            RETURNING *
        "#,
        invitation.message,
        invitation_key,
        current_user.id,
        invitation.receiver_email
    )
    .fetch_one(pool)
    .await;

    match sent_invitation {
        Ok(_) => Ok(sent_invitation.unwrap()),
        Err(e) => {
            println!("{:#?}", e);
            match e {
                sqlx::Error::Database(db_error) => db_error.message().to_string(),
                _ => e.to_string(),
            };
            Err(format!("could not send invite").into())
        }
    }
}

pub async fn does_unexpired_invitation_exist(
    pool: &PgPool,
    invitation_key: &str,
) -> Result<Invitation, Box<dyn Error>> {
    let invitation = sqlx::query_as!(
        Invitation,
        r#"
           SELECT * FROM invitations
           WHERE invitation_key = $1
           AND expires_at > CURRENT_TIMESTAMP
        "#,
        invitation_key
    )
    .fetch_one(pool)
    .await;

    match invitation {
        Ok(_) => Ok(invitation.unwrap()),
        Err(e) => {
            println!("{:#?}", e);
            match e {
                sqlx::Error::Database(db_error) => db_error.message().to_string(),
                _ => e.to_string(),
            };
            Err(format!("invitation not found").into())
        }
    }
}

pub async fn set_invitations_available(
    pool: &PgPool,
    amount: i16,
    current_user: &User,
) -> Result<(), Box<dyn Error>> {
    let result = sqlx::query!(
        r#"
           UPDATE users SET invitations = $1
           WHERE id = $2
        "#,
        amount,
        current_user.id
    )
    .execute(pool)
    .await;

    match result {
        Ok(_) => Ok(()),
        Err(e) => {
            println!("{:#?}", e);
            match e {
                sqlx::Error::Database(db_error) => db_error.message().to_string(),
                _ => e.to_string(),
            };
            Err(format!("could not set invitation amount").into())
        }
    }
}
