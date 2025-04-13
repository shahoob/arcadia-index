use std::str::FromStr;

use actix_multipart::form::{MultipartForm, bytes::Bytes, text::Text};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{prelude::FromRow, types::Json};
use utoipa::{ToResponse, ToSchema};

#[derive(Debug, Deserialize, Serialize, sqlx::Type)]
#[sqlx(type_name = "audio_codec_enum")]
#[sqlx(rename_all = "lowercase")]
pub enum AudioCodec {
    Mp2,
    Mp3,
    Aac,
    Ac3,
    Dts,
    Flac,
    Pcm,
    #[sqlx(rename = "true-hd")]
    TrueHd,
    Opus,
    Dsd,
}

#[derive(Debug, Deserialize, Serialize, sqlx::Type)]
#[sqlx(type_name = "audio_bitrate_sampling_enum")]
pub enum AudioBitrateSampling {
    #[sqlx(rename = "192")]
    Bitrate192,
    #[sqlx(rename = "256")]
    Bitrate256,
    #[sqlx(rename = "320")]
    Bitrate320,
    #[sqlx(rename = "APS (VBR)")]
    ApsVbr,
    #[sqlx(rename = "V2 (VBR)")]
    V2Vbr,
    #[sqlx(rename = "V1 (VBR)")]
    V1Vbr,
    #[sqlx(rename = "APX (VBR)")]
    ApxVbr,
    #[sqlx(rename = "V0 (VBR)")]
    V0Vbr,
    Lossless,
    #[sqlx(rename = "24bit Lossless")]
    Lossless24Bit,
    #[sqlx(rename = "DSD64")]
    Dsd64,
    #[sqlx(rename = "DSD128")]
    Dsd128,
    #[sqlx(rename = "DSD256")]
    Dsd256,
    #[sqlx(rename = "DSD512")]
    Dsd512,
    Other,
}

#[derive(Debug, Deserialize, Serialize, sqlx::Type)]
#[sqlx(type_name = "video_codec_enum")]
pub enum VideoCodec {
    #[sqlx(rename = "mpeg1")]
    Mpeg1,
    #[sqlx(rename = "mpeg2")]
    Mpeg2,
    Xvid,
    #[sqlx(rename = "divX")]
    DivX,
    #[sqlx(rename = "h264")]
    H264,
    #[sqlx(rename = "h265")]
    H265,
    #[sqlx(rename = "vc-1")]
    Vc1,
    #[sqlx(rename = "vp9")]
    Vp9,
    BD50,
    UHD100,
}

#[derive(Debug, Deserialize, Serialize, sqlx::Type)]
#[sqlx(type_name = "features_enum")]
pub enum Features {
    HDR,
    DV,
    Commentary,
    Remux,
    #[sqlx(rename = "3D")]
    ThreeD,
    Booklet,
    Cue,
}
impl FromStr for Features {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "HDR" => Ok(Features::HDR),
            "DV" => Ok(Features::DV),
            "Commentary" => Ok(Features::Commentary),
            "Remux" => Ok(Features::Remux),
            "3D" => Ok(Features::ThreeD),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Serialize, FromRow)]
pub struct Torrent {
    pub id: i64,
    pub edition_group_id: i64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub created_by_id: i64,
    pub release_name: Option<String>,
    pub release_group: String,
    pub description: Option<String>,       // specific to the torrent
    pub file_amount_per_type: Json<Value>, // (5 mp3, 1 log, 5 jpg, etc.)
    pub uploaded_as_anonymous: bool,
    pub file_list: Json<Value>,
    pub mediainfo: String,
    pub trumpable: Option<String>, // description of why it is trumpable
    pub staff_checked: bool,
    pub language: Option<String>, // (fallback to original language) (english, french, etc.)
    pub container: String, // container of the main file (ex: if mkv movie and srt subs, mkv is the main)
    pub size: i64,         // in bytes
    // ---- audio
    pub duration: Option<i32>, // in seconds
    pub audio_codec: Option<AudioCodec>,
    pub audio_bitrate: Option<i32>, // in kb/s
    pub audio_bitrate_sampling: Option<AudioBitrateSampling>,
    pub audio_channels: Option<String>,
    // ---- audio
    // ---- video
    pub video_codec: Option<VideoCodec>,
    pub features: Option<Vec<Features>>,
    pub subtitle_languages: Option<Vec<String>>,
    pub video_resolution: Option<String>, // ---- video
}

#[derive(Debug, MultipartForm, FromRow)]
pub struct UploadedTorrent {
    pub release_name: Text<String>,
    pub release_group: Text<String>,
    pub description: Option<Text<String>>, // specific to the torrent
    pub uploaded_as_anonymous: Text<bool>,
    pub mediainfo: Text<String>,
    pub torrent_file: Bytes,
    pub language: Option<Text<String>>, // (fallback to original language) (english, french, etc.)
    pub container: Text<String>,
    // one of them should be given
    pub edition_group_id: Text<i64>,
    // pub edition_group: Option<UserCreatedEditionGroup>,
    // ---- audio
    pub duration: Option<Text<i32>>, // in seconds
    pub audio_codec: Option<Text<AudioCodec>>,
    pub audio_bitrate: Option<Text<i32>>, // in kb/s
    pub audio_channels: Option<Text<String>>,
    pub audio_bitrate_sampling: Option<Text<AudioBitrateSampling>>,
    // ---- audio
    // ---- video
    pub video_codec: Option<Text<VideoCodec>>,
    pub features: Text<String>,
    pub subtitle_languages: Text<String>,
    pub video_resolution: Option<Text<String>>, // ---- video
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct TorrentSearch {
    pub title_group_name: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct LiteTorrent {
    pub id: i64,
    pub edition_group_id: i64,
    pub created_at: NaiveDateTime,
    pub release_name: Option<String>,
    pub file_amount_per_type: Json<Value>,
    pub trumpable: Option<String>,
    pub staff_checked: bool,
    pub language: Option<String>,
    pub container: String,
    pub size: i64,
    pub duration: Option<i32>,
    pub audio_codec: Option<AudioCodec>,
    pub audio_bitrate: Option<i32>,
    pub audio_bitrate_sampling: Option<AudioBitrateSampling>,
    pub audio_channels: Option<String>,
    pub video_codec: Option<VideoCodec>,
    pub features: Option<Vec<Features>>,
    pub subtitle_languages: Option<Vec<String>>,
    pub video_resolution: Option<String>,
}
