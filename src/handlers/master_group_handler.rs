use crate::{
    Arcadia, Result,
    models::{master_group::UserCreatedMasterGroup, user::User},
    repositories::master_group_repository::create_master_group,
};
use actix_web::{HttpResponse, web};

pub async fn add_master_group(
    form: web::Json<UserCreatedMasterGroup>,
    arc: web::Data<Arcadia>,
    current_user: User,
) -> Result<HttpResponse> {
    let master_group = create_master_group(&arc.pool, &form, &current_user).await?;

    Ok(HttpResponse::Created().json(master_group))
}
