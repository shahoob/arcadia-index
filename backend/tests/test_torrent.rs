use actix_web::{http::StatusCode, test};
use serde::Deserialize;
use sqlx::PgPool;

pub mod common;

#[sqlx::test(fixtures(
    "with_test_user",
    "with_test_title_group",
    "with_test_edition_group",
    "with_test_torrent"
))]
async fn test_valid_torrent(pool: PgPool) {
    let (service, token) = common::create_test_app_and_login(pool, 1.0, 1.0).await;

    let req = test::TestRequest::get()
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .insert_header(token)
        .uri("/api/torrent?id=1")
        .to_request();

    let resp = test::call_service(&service, req).await;

    assert_eq!(resp.status(), StatusCode::OK);

    // A minimum set of definitions for assertions.
    #[derive(Debug, Deserialize)]
    struct Info {
        private: isize,
    }

    #[derive(Debug, Deserialize)]
    struct MetaInfo {
        info: Info,
        announce: String,
    }

    let metainfo = common::read_body_bencode::<MetaInfo, _>(resp)
        .await
        .expect("could not deserialize metainfo");

    assert_eq!(
        metainfo.info.private, 1,
        "expected downloaded torrent to be private"
    );

    let test_user_passkey = "d2037c66dd3e13044e0d2f9b891c3837";
    assert!(
        metainfo.announce.contains(test_user_passkey),
        "expected announce url to contain test_user passkey"
    );
}

#[sqlx::test(fixtures("with_test_user", "with_test_title_group", "with_test_edition_group",))]
async fn test_upload_torrent(pool: PgPool) {
    use actix_multipart_rfc7578::client::multipart;

    let mut form = multipart::Form::default();

    form.add_text("release_name", "test release name");
    form.add_text("release_group", "TESTGRoUP");
    form.add_text("description", "This is a test description");
    form.add_text("uploaded_as_anonymous", "true");
    form.add_text("mediainfo", "test mediainfo");
    form.add_text("languages", "English");
    form.add_text("container", "MKV");
    form.add_text("edition_group_id", "1");
    form.add_text("duration", "3600");
    form.add_text("audio_codec", "flac");
    form.add_text("audio_bitrate", "1200");
    form.add_text("audio_channels", "5.1");
    form.add_text("audio_bitrate_sampling", "256");
    form.add_text("video_codec", "h264");
    form.add_text("features", "DV,HDR");
    form.add_text("subtitle_languages", "English,French");
    form.add_text("video_resolution", "1080p");

    let torrent_data = bytes::Bytes::from_static(include_bytes!(
        "data/debian-12.10.0-i386-netinst.iso.torrent"
    ));

    form.add_reader_file(
        "torrent_file",
        std::io::Cursor::new(torrent_data),
        "torrent_file.torrent",
    );

    let content_type = form.content_type();

    let payload = actix_web::body::to_bytes(multipart::Body::from(form))
        .await
        .unwrap();

    let (service, token) = common::create_test_app_and_login(pool, 1.0, 1.0).await;

    let req = test::TestRequest::post()
        .uri("/api/torrent")
        .insert_header(token)
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .insert_header(("Content-Type", content_type))
        .set_payload(payload)
        .to_request();

    #[derive(Debug, Deserialize)]
    struct Torrent {
        edition_group_id: i64,
        created_by_id: i64,
    }

    let torrent = common::call_and_read_body_json_with_status::<Torrent, _>(
        &service,
        req,
        StatusCode::CREATED,
    )
    .await;

    assert_eq!(torrent.edition_group_id, 1);
    assert_eq!(torrent.created_by_id, 1);
}
