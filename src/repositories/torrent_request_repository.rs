use crate::models::{
    torrent_request::{TorrentRequest, UserCreatedTorrentRequest},
    user::User,
};
use actix_web::web;
use sqlx::PgPool;
use std::error::Error;

pub async fn create_torrent_request(
    pool: &web::Data<PgPool>,
    torrent_request: &UserCreatedTorrentRequest,
    current_user: &User,
) -> Result<TorrentRequest, Box<dyn Error>> {
    let create_torrent_request_query = r#"
        INSERT INTO torrent_requests 
        (title_group_id, created_by_id, edition_name,
        release_group, description, language, container, audio_codec,
        audio_channels, video_codec, features, subtitle_languages, video_resolution,
        bounty_upload, bounty_bonus_points) 
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8::audio_codec_enum, $9,
        $10::video_codec_enum, $11::features_enum[], $12, $13, $14, $15)
        RETURNING *;
    "#;

    let created_torrent_request = sqlx::query_as::<_, TorrentRequest>(create_torrent_request_query)
        .bind(&torrent_request.title_group_id)
        .bind(&current_user.id)
        .bind(&torrent_request.edition_name)
        .bind(&torrent_request.release_group)
        .bind(&torrent_request.description)
        .bind(&torrent_request.language)
        .bind(&torrent_request.container)
        .bind(&torrent_request.audio_codec)
        .bind(&torrent_request.audio_channels)
        .bind(&torrent_request.video_codec)
        .bind(&torrent_request.features)
        .bind(&torrent_request.subtitle_languages)
        .bind(&torrent_request.video_resolution)
        .bind(&torrent_request.bounty_upload)
        .bind(&torrent_request.bounty_bonus_points)
        .fetch_one(pool.get_ref())
        .await;

    match created_torrent_request {
        Ok(_) => Ok(created_torrent_request.unwrap()),
        Err(e) => {
            println!("{:#?}", e);
            match e {
                sqlx::Error::Database(db_error) => db_error.message().to_string(),
                _ => e.to_string(),
            };
            Err(format!("could not create torrent request").into())
        }
    }
}
