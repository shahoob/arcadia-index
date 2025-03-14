use actix_web::{HttpResponse, web};
use sqlx::PgPool;

use crate::models::{torrent::UploadedTorrent, user::User};

// pub async fn upload_torrent(
//     form: web::Form<UploadedTorrent>,
//     pool: web::Data<PgPool>,
//     current_user: User,
// ) -> HttpResponse {
//     // TODO : check if user can upload

//     match create_torrent(&pool, &form, &current_user).await {
//         Ok(user) => HttpResponse::Created().json(serde_json::json!(user)),
//         Err(err) => HttpResponse::InternalServerError().json(serde_json::json!({
//             "error": err.to_string()
//         })),
//     }
// }
