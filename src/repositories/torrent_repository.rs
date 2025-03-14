use std::error::Error;

use actix_web::web;
use rand::{
    distr::{Alphanumeric, SampleString},
    rng,
};
use sqlx::PgPool;

use crate::models::{
    torrent::{Torrent, UploadedTorrent},
    user::User,
};

// pub async fn create_torrent(
//     pool: &web::Data<PgPool>,
//     torrent_form: &UploadedTorrent,
//     current_user: &User,
// ) -> Result<Torrent, Box<dyn Error>> {
//     let create_torrent_query = r#"
//     "#;

//     // match set_invitations_available(pool, current_user.invitations - 1, current_user).await {
//     //     Ok(_) => {}
//     //     Err(_) => {
//     //         return Err(format!("could not remove invite from counter").into());
//     //     }
//     // }

//     let uploaded_torrent = sqlx::query_as::<_, Torrent>(create_torrent_query)
//         .bind(&invitation.message)
//         .bind(&invitation_key)
//         .bind(&current_user.id)
//         .bind(&invitation.receiver_email)
//         .bind(&expires_at)
//         .fetch_one(pool.get_ref())
//         .await;

//     match uploaded_torrent {
//         Ok(_) => Ok(uploaded_torrent.unwrap()),
//         Err(e) => {
//             println!("{:#?}", e);
//             match e {
//                 sqlx::Error::Database(db_error) => db_error.message().to_string(),
//                 _ => e.to_string(),
//             };
//             Err(format!("could not send invite").into())
//         }
//     }
// }
