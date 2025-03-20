use std::str::FromStr;

use actix_multipart::form::{MultipartForm, bytes::Bytes, text::Text};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{prelude::FromRow, types::Json};

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
    #[sqlx(rename = "H265")]
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
    pub id: i32,
    pub edition_group_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub created_by_id: i32,
    pub release_name: Option<String>,
    pub release_group: String,
    pub description: Option<String>,       // specific to the torrent
    pub file_amount_per_type: Json<Value>, // (5 mp3, 1 log, 5 jpg, etc.)
    pub uploaded_as_anonymous: bool,
    pub file_list: Json<Value>,
    pub mediainfo: String,
    pub trumpable: Option<String>, // description of why it is trumpable
    pub staff_checked: bool,
    pub size: i64, // in bytes
    // ---- audio
    pub duration: i32, // in seconds
    pub audio_codec: AudioCodec,
    pub audio_bitrate: i32, // in kb/s
    pub audio_bitrate_sampling: AudioBitrateSampling,
    pub audio_channels: String,
    // ---- audio
    // ---- video
    pub video_codec: VideoCodec,
    pub features: Vec<Features>,
    pub subtitle_languages: Vec<String>,
    // ---- video
}

#[derive(Debug, MultipartForm, FromRow)]
pub struct UploadedTorrent {
    pub release_name: Text<String>,
    pub release_group: Text<String>,
    pub description: Text<Option<String>>, // specific to the torrent
    pub uploaded_as_anonymous: Text<bool>,
    pub mediainfo: Text<String>,
    pub torrent_file: Bytes,
    // one of them should be given
    pub edition_group_id: Text<i32>,
    // pub edition_group: Option<UserCreatedEditionGroup>,
    // ---- audio
    pub duration: Text<i32>, // in seconds
    pub audio_codec: Text<AudioCodec>,
    pub audio_bitrate: Text<i32>, // in kb/s
    pub audio_channels: Text<String>,
    pub audio_bitrate_sampling: Text<AudioBitrateSampling>,
    // ---- audio
    // ---- video
    pub video_codec: Text<VideoCodec>,
    pub features: Text<String>,
    pub subtitle_languages: Text<String>,
    // ---- video
}
