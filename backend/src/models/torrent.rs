use std::str::FromStr;

use actix_multipart::form::{MultipartForm, bytes::Bytes, text::Text};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{prelude::FromRow, types::Json};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, Serialize, sqlx::Type, ToSchema)]
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
    #[serde(alias = "true-hd")]
    TrueHd,
    Opus,
    Dsd,
}

#[derive(Debug, Deserialize, Serialize, sqlx::Type, ToSchema)]
#[sqlx(type_name = "audio_channels_enum")]
pub enum AudioChannels {
    #[sqlx(rename = "1.0")]
    #[serde(alias = "1.0")]
    OneDotZero,
    #[sqlx(rename = "2.0")]
    #[serde(alias = "2.0")]
    TwoDotZero,
    #[sqlx(rename = "2.1")]
    #[serde(alias = "2.1")]
    TwoDotOne,
    #[sqlx(rename = "5.0")]
    #[serde(alias = "5.0")]
    FiveDotZero,
    #[sqlx(rename = "5.1")]
    #[serde(alias = "5.1")]
    FiveDotOne,
    #[sqlx(rename = "7.1")]
    #[serde(alias = "7.1")]
    SevenDotOne,
}

#[derive(Debug, Deserialize, Serialize, sqlx::Type, ToSchema)]
#[sqlx(type_name = "audio_bitrate_sampling_enum")]
pub enum AudioBitrateSampling {
    #[sqlx(rename = "192")]
    #[serde(alias = "192")]
    Bitrate192,
    #[sqlx(rename = "256")]
    #[serde(alias = "256")]
    Bitrate256,
    #[sqlx(rename = "320")]
    #[serde(alias = "320")]
    Bitrate320,
    #[sqlx(rename = "APS (VBR)")]
    #[serde(alias = "APS (VBR)")]
    ApsVbr,
    #[sqlx(rename = "V2 (VBR)")]
    #[serde(alias = "V2 (VBR)")]
    V2Vbr,
    #[sqlx(rename = "V1 (VBR)")]
    #[serde(alias = "V1 (VBR)")]
    V1Vbr,
    #[sqlx(rename = "APX (VBR)")]
    #[serde(alias = "APX (VBR)")]
    ApxVbr,
    #[sqlx(rename = "V0 (VBR)")]
    #[serde(alias = "V0 (VBR)")]
    V0Vbr,
    Lossless,
    #[sqlx(rename = "24bit Lossless")]
    #[serde(alias = "24bit Lossless")]
    Lossless24Bit,
    #[sqlx(rename = "DSD64")]
    #[serde(alias = "DSD64")]
    Dsd64,
    #[sqlx(rename = "DSD128")]
    #[serde(alias = "DSD128")]
    Dsd128,
    #[sqlx(rename = "DSD256")]
    #[serde(alias = "DSD256")]
    Dsd256,
    #[sqlx(rename = "DSD512")]
    #[serde(alias = "DSD512")]
    Dsd512,
    Other,
}

#[derive(Debug, Deserialize, Serialize, sqlx::Type, ToSchema)]
#[sqlx(type_name = "video_codec_enum")]
pub enum VideoCodec {
    #[sqlx(rename = "mpeg1")]
    #[serde(alias = "mpeg1")]
    Mpeg1,
    #[sqlx(rename = "mpeg2")]
    #[serde(alias = "mpeg2")]
    Mpeg2,
    Xvid,
    #[sqlx(rename = "divX")]
    #[serde(alias = "divX")]
    DivX,
    #[sqlx(rename = "h264")]
    #[serde(alias = "h264")]
    H264,
    #[sqlx(rename = "h265")]
    #[serde(alias = "h265")]
    H265,
    #[sqlx(rename = "vc-1")]
    #[serde(alias = "vc-1")]
    Vc1,
    #[sqlx(rename = "vp9")]
    #[serde(alias = "vp9")]
    Vp9,
    BD50,
    UHD100,
}

#[derive(Debug, Deserialize, Serialize, sqlx::Type, ToSchema)]
#[sqlx(type_name = "features_enum")]
pub enum Features {
    #[sqlx(rename = "HDR")]
    #[serde(alias = "HDR")]
    Hdr,
    #[sqlx(rename = "DV")]
    #[serde(alias = "DV")]
    Dv,
    Commentary,
    Remux,
    #[sqlx(rename = "3D")]
    #[serde(alias = "3D")]
    ThreeD,
    Booklet,
    Cue,
}

impl FromStr for Features {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "HDR" => Ok(Features::Hdr),
            "DV" => Ok(Features::Dv),
            "Commentary" => Ok(Features::Commentary),
            "Remux" => Ok(Features::Remux),
            "3D" => Ok(Features::ThreeD),
            "Booklet" => Ok(Features::Booklet),
            "Cue" => Ok(Features::Cue),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Serialize, FromRow, ToSchema)]
pub struct Torrent {
    pub id: i64,
    pub edition_group_id: i64,
    #[schema(value_type = String, format = DateTime)]
    pub created_at: NaiveDateTime,
    #[schema(value_type = String, format = DateTime)]
    pub updated_at: NaiveDateTime,
    pub created_by_id: i64,
    pub release_name: Option<String>,
    pub release_group: Option<String>,
    pub description: Option<String>, // specific to the torrent
    #[schema(value_type = Value)]
    pub file_amount_per_type: Json<Value>, // (5 mp3, 1 log, 5 jpg, etc.)
    pub uploaded_as_anonymous: bool,
    #[schema(value_type = Value)]
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
    pub audio_channels: Option<AudioChannels>,
    // ---- audio
    // ---- video
    pub video_codec: Option<VideoCodec>,
    pub features: Option<Vec<Features>>,
    pub subtitle_languages: Option<Vec<String>>,
    pub video_resolution: Option<String>, // ---- video
}

#[derive(Debug, MultipartForm, FromRow, ToSchema)]
pub struct UploadedTorrent {
    #[schema(value_type = String)]
    pub release_name: Text<String>,
    #[schema(value_type = String)]
    pub release_group: Option<Text<String>>,
    #[schema(value_type = String)]
    pub description: Option<Text<String>>, // specific to the torrent
    #[schema(value_type = bool)]
    pub uploaded_as_anonymous: Text<bool>,
    #[schema(value_type = String)]
    pub mediainfo: Text<String>,
    #[schema(value_type = String, format = Binary, content_media_type = "application/octet-stream")]
    pub torrent_file: Bytes,
    #[schema(value_type = String)]
    pub language: Option<Text<String>>, // (fallback to original language) (english, french, etc.)
    #[schema(value_type = String)]
    pub container: Text<String>,
    // one of them should be given
    #[schema(value_type = i64)]
    pub edition_group_id: Text<i64>,
    // pub edition_group: Option<UserCreatedEditionGroup>,
    // ---- audio
    #[schema(value_type = i32)]
    pub duration: Option<Text<i32>>, // in seconds
    #[schema(value_type = AudioCodec)]
    pub audio_codec: Option<Text<AudioCodec>>,
    #[schema(value_type = i32)]
    pub audio_bitrate: Option<Text<i32>>, // in kb/s
    #[schema(value_type = String)]
    pub audio_channels: Option<Text<AudioChannels>>,
    #[schema(value_type = AudioBitrateSampling)]
    pub audio_bitrate_sampling: Option<Text<AudioBitrateSampling>>,
    // ---- audio
    // ---- video
    #[schema(value_type = VideoCodec)]
    pub video_codec: Option<Text<VideoCodec>>,
    #[schema(value_type = String)]
    pub features: Text<String>,
    #[schema(value_type = String)]
    pub subtitle_languages: Text<String>,
    #[schema(value_type = String)]
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
