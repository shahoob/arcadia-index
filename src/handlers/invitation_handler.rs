use crate::{
    Arcadia, Error, Result,
    models::{invitation::SentInvitation, user::User},
    repositories::invitation_repository::create_invitation,
};
use actix_web::{HttpResponse, web};

pub async fn send_invitation(
    invitation: web::Json<SentInvitation>,
    arc: web::Data<Arcadia>,
    current_user: User,
) -> Result<HttpResponse> {
    if current_user.invitations == 0 {
        return Err(Error::NoInvitationsAvailable);
    }

    // TODO: send email to the user who receives the invite

    let invitation = create_invitation(&arc.pool, &invitation, &current_user).await?;

    Ok(HttpResponse::Created().json(invitation))
}
