use std::error::Error;

use actix_web::web;
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
    pool: &web::Data<PgPool>,
    invitation: &SentInvitation,
    current_user: &User,
) -> Result<Invitation, Box<dyn Error>> {
    let create_invite_query = r#"
        INSERT INTO invitations (message, invitation_key, sender_id, receiver_email, expires_at) 
        VALUES ($1, $2, $3, $4, $5)
        RETURNING *
    "#;

    // TODO: make this duration configurable
    let expires_at = chrono::Utc::now() + chrono::Duration::days(3);

    let invitation_key: String = Alphanumeric.sample_string(&mut rng(), 50);

    match set_invitations_available(pool, current_user.invitations - 1, current_user).await {
        Ok(_) => {}
        Err(_) => {
            return Err(format!("could not remove invite from counter").into());
        }
    }

    let sent_invitation = sqlx::query_as::<_, Invitation>(create_invite_query)
        .bind(&invitation.message)
        .bind(&invitation_key)
        .bind(&current_user.id)
        .bind(&invitation.receiver_email)
        .bind(&expires_at)
        .fetch_one(pool.get_ref())
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

pub async fn does_invitation_exist(
    pool: &web::Data<PgPool>,
    invitation_key: &str,
) -> Result<Invitation, Box<dyn Error>> {
    let query = r#"
       SELECT * FROM invitations WHERE invitation_key = $1;
    "#;

    let invitation = sqlx::query_as::<_, Invitation>(query)
        .bind(&invitation_key)
        .fetch_one(pool.get_ref())
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
    pool: &web::Data<PgPool>,
    amount: i16,
    current_user: &User,
) -> Result<(), Box<dyn Error>> {
    let set_invitation_amount_query = r#"
       UPDATE users SET invitations = $1 WHERE id = $2;
    "#;

    let result = sqlx::query(set_invitation_amount_query)
        .bind(&amount)
        .bind(&current_user.id)
        .execute(pool.get_ref())
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
