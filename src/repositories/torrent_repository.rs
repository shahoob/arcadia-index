use crate::{
    Error, Result,
    models::{
        torrent::{Features, Torrent, UploadedTorrent},
        user::User,
    },
};

use bip_metainfo::Metainfo;
use reqwest::Url;
use serde_json::json;
use sqlx::PgPool;
use std::{fs, path::PathBuf, str::FromStr};

use super::notification_repository::notify_users;

#[derive(sqlx::FromRow)]
struct LiteTitleGroupInfo {
    id: i64,
    name: String,
}

pub async fn create_torrent(
    pool: &PgPool,
    torrent_form: &UploadedTorrent,
    current_user: &User,
    frontend_url: &Url,
    dottorrent_files_path: &PathBuf,
) -> Result<Torrent> {
    let create_torrent_query = r#"
    INSERT INTO torrents (
        edition_group_id, created_by_id, release_name, 
        release_group, description, file_amount_per_type, uploaded_as_anonymous, 
        file_list, mediainfo, trumpable, staff_checked, size,
        duration, audio_codec, audio_bitrate, audio_bitrate_sampling,
        audio_channels, video_codec, features, subtitle_languages, video_resolution, container,
        language
    ) VALUES (
        $1, $2, $3, 
        $4, $5, $6, $7, 
        $8, $9, $10, $11, $12,
        $13, $14::audio_codec_enum, $15, $16::audio_bitrate_sampling_enum,
        $17, $18::video_codec_enum, $19::features_enum[], $20, $21, $22, $23
    ) RETURNING *;
    "#;

    let metainfo =
        Metainfo::from_bytes::<Vec<u8>>(torrent_form.torrent_file.data.clone().into()).unwrap();
    // let file_list = metainfo
    //     .info()
    //     .files()
    //     .map(|file| {
    //         let dir = metainfo.info().directory();
    //         let file_path = file.path().to_str().unwrap();
    //         if !dir.is_none() {
    //             format!("{}/{}", dir.unwrap().to_str().unwrap(), file_path)
    //         } else {
    //             file_path.to_string()
    //         }
    //     })
    //     .collect::<Vec<String>>();

    let info = metainfo.info();
    // TODO: torrent metadata extraction should be done on the client side
    let parent_folder = info.directory().map(|d| d.to_str().unwrap()).unwrap_or("");
    let files = info
        .files()
        .map(|f| json!({"name": f.path().to_str().unwrap(), "size": f.length()}))
        .collect::<Vec<_>>();

    let file_list = json!({"parent_folder": parent_folder, "files": files});

    let file_amount_per_type = json!(
        metainfo
            .info()
            .files()
            .flat_map(|file| file.path().to_str().unwrap().split('.').last())
            .fold(std::collections::HashMap::new(), |mut acc, ext| {
                *acc.entry(ext.to_string()).or_insert(0) += 1;
                acc
            })
    );

    // TODO: check if the torrent is trumpable: via a service ?
    let trumpable = String::from("");
    let size = metainfo
        .info()
        .files()
        .map(|file| file.length())
        .sum::<u64>() as i64;

    let uploaded_torrent = sqlx::query_as::<_, Torrent>(create_torrent_query)
        .bind(&torrent_form.edition_group_id.0)
        .bind(&current_user.id)
        .bind(&*torrent_form.release_name.0)
        .bind(&*torrent_form.release_group.0)
        .bind(torrent_form.description.as_deref())
        .bind(&file_amount_per_type)
        .bind(&torrent_form.uploaded_as_anonymous.0)
        .bind(&file_list)
        .bind(&*torrent_form.mediainfo.0)
        .bind(&trumpable)
        .bind(&false)
        .bind(&size)
        .bind(torrent_form.duration.as_deref())
        .bind(torrent_form.audio_codec.as_deref())
        .bind(torrent_form.audio_bitrate.as_deref())
        .bind(torrent_form.audio_bitrate_sampling.as_deref())
        .bind(torrent_form.audio_channels.as_deref())
        .bind(torrent_form.video_codec.as_deref())
        .bind(if torrent_form.features.is_empty() {
            Vec::new()
        } else {
            torrent_form
                .features
                .0
                .split(',')
                .map(|f| Features::from_str(f).ok().unwrap())
                .collect::<Vec<Features>>()
        })
        .bind(if torrent_form.subtitle_languages.is_empty() {
            Vec::new()
        } else {
            torrent_form
                .subtitle_languages
                .0
                .split(',')
                .map(|f| f.trim())
                .collect::<Vec<&str>>()
        })
        .bind(torrent_form.video_resolution.as_deref())
        .bind(&*torrent_form.container)
        .bind(torrent_form.language.as_deref())
        .fetch_one(pool)
        .await
        .map_err(Error::CouldNotCreateTorrent)?;

    let title_group_info = sqlx::query_as!(
        LiteTitleGroupInfo,
        r#"
            SELECT title_groups.id, title_groups.name
            FROM edition_groups
            JOIN title_groups ON edition_groups.title_group_id = title_groups.id
            WHERE edition_groups.id = $1
        "#,
        torrent_form.edition_group_id.0
    )
    .fetch_one(pool)
    .await?;

    //TODO: edit torrent file : remove announce url, add comment with torrent url, etc.
    let output_path = format!(
        "{}",
        dottorrent_files_path
            .join(format!("{}{}", uploaded_torrent.id, ".torrent"))
            .to_str()
            .unwrap()
    );
    fs::write(&output_path, &torrent_form.torrent_file.data)
        .map_err(|error| Error::CouldNotSaveTorrentFile(output_path, error.to_string()))?;

    let _ = notify_users(
        pool,
        &"torrent_uploaded",
        &title_group_info.id,
        &"New torrent uploaded subscribed title group",
        &format!(
            "New torrent uploaded in title group \"{}\"",
            title_group_info.name
        ),
    )
    .await;

    Ok(uploaded_torrent)
}
