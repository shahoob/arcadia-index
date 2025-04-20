use crate::{
    Error, Result,
    models::{
        torrent_request::{TorrentRequest, UserCreatedTorrentRequest},
        user::User,
    },
};
use sqlx::PgPool;

pub async fn create_torrent_request(
    pool: &PgPool,
    torrent_request: &UserCreatedTorrentRequest,
    current_user: &User,
) -> Result<TorrentRequest> {
    let create_torrent_request_query = r#"
        INSERT INTO torrent_requests
        (title_group_id, created_by_id, edition_name,
        release_group, description, languages, container, audio_codec,
        audio_channels, video_codec, features, subtitle_languages, video_resolution,
        bounty_upload, bounty_bonus_points)
        VALUES ($1, $2, $3, $4, $5, $6::language_enum[], $7, $8::audio_codec_enum, $9,
        $10::video_codec_enum, $11::features_enum[], $12, $13, $14, $15)
        RETURNING *;
    "#;

    let created_torrent_request = sqlx::query_as::<_, TorrentRequest>(create_torrent_request_query)
        .bind(torrent_request.title_group_id)
        .bind(current_user.id)
        .bind(&torrent_request.edition_name)
        .bind(&torrent_request.release_group)
        .bind(&torrent_request.description)
        .bind(&torrent_request.languages)
        .bind(&torrent_request.container)
        .bind(&torrent_request.audio_codec)
        .bind(&torrent_request.audio_channels)
        .bind(&torrent_request.video_codec)
        .bind(&torrent_request.features)
        .bind(&torrent_request.subtitle_languages)
        .bind(&torrent_request.video_resolution)
        .bind(torrent_request.bounty_upload)
        .bind(torrent_request.bounty_bonus_points)
        .fetch_one(pool)
        .await
        .map_err(Error::CouldNotCreateTorrentRequest)?;

    Ok(created_torrent_request)
}
