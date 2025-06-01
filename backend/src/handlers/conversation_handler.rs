use crate::{
    Arcadia, Result,
    models::{
        conversation::{
            Conversation, ConversationHierarchy, ConversationMessage, UserCreatedConversation,
            UserCreatedConversationMessage,
        },
        user::User,
    },
    repositories::conversation_repository::{
        create_conversation, create_conversation_message, find_conversation,
    },
};
use actix_web::{HttpResponse, web};
use serde::Deserialize;
use utoipa::IntoParams;

#[utoipa::path(
    post,
    path = "/api/conversation",
    responses(
        (status = 200, description = "Successfully created the conversation and first message", body=Conversation),
    )
)]
pub async fn add_conversation(
    mut conversation: web::Json<UserCreatedConversation>,
    arc: web::Data<Arcadia>,
    current_user: User,
) -> Result<HttpResponse> {
    // creates a conversation and the first message, empty conversations should not be allowed
    let conversation = create_conversation(&arc.pool, &mut conversation, current_user.id).await?;

    Ok(HttpResponse::Created().json(conversation))
}

#[derive(Debug, Deserialize, IntoParams)]
pub struct GetConversationQuery {
    id: i64,
}

#[utoipa::path(
    get,
    params(GetConversationQuery),
    path = "/api/conversation",
    responses(
        (status = 200, description = "Found the conversation and its messages", body=ConversationHierarchy),
    )
)]
pub async fn get_conversation(
    query: web::Query<GetConversationQuery>,
    arc: web::Data<Arcadia>,
    current_user: User,
) -> Result<HttpResponse> {
    let conversation_with_messages =
        find_conversation(&arc.pool, query.id, current_user.id).await?;

    Ok(HttpResponse::Created().json(conversation_with_messages))
}

#[utoipa::path(
    post,
    path = "/api/conversation/message",
    responses(
        (status = 200, description = "Successfully created the conversation's message", body=ConversationMessage),
    )
)]
pub async fn add_conversation_message(
    message: web::Json<UserCreatedConversationMessage>,
    arc: web::Data<Arcadia>,
    current_user: User,
) -> Result<HttpResponse> {
    let message = create_conversation_message(&arc.pool, &message, current_user.id).await?;

    Ok(HttpResponse::Created().json(message))
}
