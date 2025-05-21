use crate::{
    Arcadia, Result,
    models::{
        forum::{ForumOverview, ForumPost, UserCreatedForumPost},
        user::User,
    },
    repositories::forum_repository::{create_forum_post, find_forum_overview},
};
use actix_web::{HttpResponse, web};

#[utoipa::path(
    post,
    path = "/api/forum/post",
    responses(
        (status = 200, description = "Successfully created the forum post", body=ForumPost),
    )
)]
pub async fn add_forum_post(
    forum_post: web::Json<UserCreatedForumPost>,
    arc: web::Data<Arcadia>,
    current_user: User,
) -> Result<HttpResponse> {
    let forum_post = create_forum_post(&arc.pool, &forum_post, current_user.id).await?;

    Ok(HttpResponse::Created().json(forum_post))
}

#[utoipa::path(
    get,
    path = "/api/forum",
    responses(
        (status = 200, description = "Returns an overview of the forum", body=ForumOverview),
    )
)]
pub async fn get_forum(arc: web::Data<Arcadia>) -> Result<HttpResponse> {
    //TODO: restrict access to some sub_categories based on forbidden_classes
    let forum_post = find_forum_overview(&arc.pool).await?;

    Ok(HttpResponse::Ok().json(forum_post))
}
