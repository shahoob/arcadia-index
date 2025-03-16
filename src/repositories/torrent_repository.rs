use std::{error::Error, fs::File, io::Read};

use actix_web::{dev::ResourcePath, web};
use bip_metainfo::Metainfo;
use std::str::FromStr;

use serde_json::json;
use sqlx::PgPool;

use crate::models::{
    torrent::{Features, Torrent, UploadedTorrent},
    user::User,
};

pub async fn create_torrent(
    pool: &web::Data<PgPool>,
    torrent_form: &UploadedTorrent,
    current_user: &User,
) -> Result<Torrent, Box<dyn Error>> {
    let create_torrent_query = r#"
    INSERT INTO torrents (
        edition_group, created_by, release_name, 
        release_group, description, file_amount_per_type, uploaded_as_anonymous, 
        file_list, mediainfo, trumpable, staff_checked, size,
        duration, audio_codec, audio_bitrate, audio_bitrate_sampling,
        audio_channels, video_codec, features, subtitle_languages
    ) VALUES (
        $1, $2, $3, 
        $4, $5, $6, $7, 
        $8, $9, $10, $11, $12,
        $13, $14::audio_codec_enum, $15, $16::audio_bitrate_sampling_enum,
        $17, $18::video_codec_enum, $19::features_enum[], $20
    ) RETURNING *;
    "#;

    let metainfo =
        Metainfo::from_bytes::<Vec<u8>>(torrent_form.torrent_file.data.clone().into()).unwrap();
    let file_list = metainfo
        .info()
        .files()
        .map(|file| {
            let dir = metainfo.info().directory();
            let file_path = file.path().to_str().unwrap();
            if !dir.is_none() {
                format!("{}/{}", dir.unwrap().to_str().unwrap(), file_path)
            } else {
                file_path.to_string()
            }
        })
        .collect::<Vec<String>>();

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
        .bind(&torrent_form.description.0)
        .bind(&file_amount_per_type)
        .bind(&torrent_form.uploaded_as_anonymous.0)
        .bind(&file_list)
        .bind(&*torrent_form.mediainfo.0)
        .bind(&trumpable)
        .bind(&false)
        .bind(&size)
        .bind(&*torrent_form.duration)
        .bind(&*torrent_form.audio_codec)
        .bind(&*torrent_form.audio_bitrate)
        .bind(&*torrent_form.audio_bitrate_sampling)
        .bind(&*torrent_form.audio_channels)
        .bind(&*torrent_form.video_codec)
        .bind(
            &torrent_form
                .features
                .0
                .split(',')
                .map(|f| Features::from_str(f).ok().unwrap())
                .collect::<Vec<Features>>(),
        )
        .bind(
            &torrent_form
                .subtitle_languages
                .0
                .split(',')
                .map(|f| f.trim())
                .collect::<Vec<&str>>(),
        )
        .fetch_one(pool.get_ref())
        .await;

    // TODO: edit the torrent file with proper flags, remove announce url and store it to the disk

    match uploaded_torrent {
        Ok(_) => Ok(uploaded_torrent.unwrap()),
        Err(e) => {
            println!("{:#?}", e);
            match e {
                sqlx::Error::Database(db_error) => db_error.message().to_string(),
                _ => e.to_string(),
            };
            Err(format!("could not create torrent").into())
        }
    }
}
