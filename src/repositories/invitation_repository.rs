use std::error::Error;

use actix_web::web;
use sqlx::PgPool;

use crate::models::{
    invitation::{Invitation, SentInvitation},
    user::User,
};

use super::user_repository::find_user_by_username;

pub async fn create_invitation(
    pool: &web::Data<PgPool>,
    invitation: &SentInvitation,
    current_user: &User,
) -> Result<Invitation, Box<dyn Error>> {
    let query = r#"
        INSERT INTO invitations (message, sender_id, receiver_id, expires_at) 
        VALUES ($1, $2, $3, $4)
        RETURNING *
    "#;

    let receiver = find_user_by_username(pool, &invitation.receiver_username)
        .await
        .unwrap();

    // TODO: make this duration configurable
    let expires_at = chrono::Utc::now() + chrono::Duration::days(3);

    let result = sqlx::query_as::<_, Invitation>(query)
        .bind(&invitation.message)
        .bind(&current_user.id)
        .bind(&receiver.id)
        .bind(&expires_at)
        .fetch_one(pool.get_ref())
        .await;

    match result {
        Ok(_) => Ok(result.unwrap()),
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
