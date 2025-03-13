use std::error::Error;

use actix_web::web;
use sqlx::PgPool;

use crate::models::invitation::{Invitation, SentInvitation};

use super::user_repository::find_user_by_username;

pub async fn create_invitation(
    pool: &web::Data<PgPool>,
    invitation: &SentInvitation,
) -> Result<Invitation, Box<dyn Error>> {
    let query = r#"
        INSERT INTO invitations (message, inviter_id, receiver_id, expires_at) 
        VALUES ($1, $2, $3, $4)
        RETURNING *
    "#;

    let receiver = find_user_by_username(pool, &invitation.receiver_username)
        .await
        .unwrap();

    let result = sqlx::query_as::<_, Invitation>(query)
        .bind(&invitation.message)
        .bind(&receiver.id)
        .bind(&receiver.id)
        .fetch_one(pool.get_ref())
        .await;

    match result {
        Ok(_) => Ok(result.unwrap()),
        Err(e) => {
            match e {
                sqlx::Error::Database(db_error) => db_error.message().to_string(),
                _ => e.to_string(),
            };
            Err(format!("User not found").into())
        }
    }
}
