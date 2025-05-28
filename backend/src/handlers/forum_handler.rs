use crate::{
    Arcadia, Result,
    models::{
        forum::{
            ForumOverview, ForumPost, ForumSubCategoryHierarchy, ForumThread, ForumThreadAndPosts,
            UserCreatedForumPost, UserCreatedForumThread,
        },
        user::User,
    },
    repositories::forum_repository::{
        create_forum_post, create_forum_thread, find_forum_overview,
        find_forum_sub_category_threads, find_forum_thread,
    },
};
use actix_web::{HttpResponse, web};
use serde::Deserialize;
use utoipa::IntoParams;

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
    post,
    path = "/api/forum/thread",
    responses(
        (status = 200, description = "Successfully created the forum thread", body=ForumThread),
    )
)]
pub async fn add_forum_thread(
    mut forum_thread: web::Json<UserCreatedForumThread>,
    arc: web::Data<Arcadia>,
    current_user: User,
) -> Result<HttpResponse> {
    let forum_thread = create_forum_thread(&arc.pool, &mut forum_thread, current_user.id).await?;

    Ok(HttpResponse::Created().json(forum_thread))
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
    let forum_overview = find_forum_overview(&arc.pool).await?;

    Ok(HttpResponse::Ok().json(forum_overview))
}

#[derive(Debug, Deserialize, IntoParams)]
pub struct GetForumSubCategoryThreadsQuery {
    id: i32,
}

#[utoipa::path(
    get,
    params(GetForumSubCategoryThreadsQuery),
    path = "/api/forum/sub-category",
    responses(
        (status = 200, description = "Returns the threads in the forum sub-category", body=ForumSubCategoryHierarchy),
    )
)]
pub async fn get_forum_sub_category_threads(
    arc: web::Data<Arcadia>,
    query: web::Query<GetForumSubCategoryThreadsQuery>,
) -> Result<HttpResponse> {
    //TODO: restrict access to some sub_categories based on forbidden_classes
    let threads = find_forum_sub_category_threads(&arc.pool, query.id).await?;

    Ok(HttpResponse::Ok().json(threads))
}

#[derive(Debug, Deserialize, IntoParams)]
pub struct GetForumThreadQuery {
    id: i64,
}

#[utoipa::path(
    get,
    params(GetForumThreadQuery),
    path = "/api/forum/thread",
    responses(
        (status = 200, description = "Returns the threads and its posts", body=ForumThreadAndPosts),
    )
)]
pub async fn get_forum_thread(
    arc: web::Data<Arcadia>,
    query: web::Query<GetForumThreadQuery>,
) -> Result<HttpResponse> {
    //TODO: restrict access to some sub_categories based on forbidden_classes
    let thread = find_forum_thread(&arc.pool, query.id).await?;

    Ok(HttpResponse::Ok().json(thread))
}
