use crate::{
    Arcadia,
    models::{invitation::SentInvitation, user::User},
    repositories::invitation_repository::create_invitation,
};
use actix_web::{HttpResponse, web};

pub async fn send_invitation(
    invitation: web::Json<SentInvitation>,
    arc: web::Data<Arcadia>,
    current_user: User,
) -> HttpResponse {
    if current_user.invitations == 0 {
        return HttpResponse::InternalServerError().json(serde_json::json!({
            "error": "you currently have 0 invitation available"
        }));
    }

    // TODO: send email to the user who receives the invite

    match create_invitation(&arc.pool, &invitation, &current_user).await {
        Ok(user) => HttpResponse::Created().json(serde_json::json!(user)),
        Err(err) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": err.to_string()
        })),
    }
}
