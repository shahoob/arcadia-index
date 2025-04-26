use actix_web::test;
use serde::Deserialize;
use sqlx::PgPool;

pub mod common;

use arcadia_backend::OpenSignups;
use arcadia_backend::tracker::announce;

#[derive(Debug, Deserialize)]
struct WrappedError {
    #[serde(rename = "failure reason")]
    _failure_reason: String,
}

#[sqlx::test(fixtures("with_test_user"))]
async fn test_announce_unknown_passkey(pool: PgPool) {
    let service = common::create_test_app(pool, OpenSignups::Enabled).await;

    let req = test::TestRequest::get()
        .uri(concat!(
            "/announce/33333333333333333333333333333333?",
            "info_hash=%7C%B3%C6y%9A%FFm%5C%3B%10%A6S%1FF%07%D9%C9%0E%C0%A7&",
            "peer_id=-lt0F01-%3D%91%BB%AC%5C%C69%C0%EDmux&",
            "key=1ab4e687&",
            "compact=1&",
            "port=6968&",
            "uploaded=0&",
            "downloaded=0&",
            "left=14&",
            "event=started"
        ))
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .to_request();

    let resp = test::call_service(&service, req).await;

    // Should fail because the passkey is invalid
    assert!(
        resp.status().is_client_error(),
        "status {} is not client error",
        resp.status()
    );

    // Any error is okay, as long as it has "failure reason" populated.
    common::read_body_bencode::<WrappedError, _>(resp)
        .await
        .expect("expected failure message");
}

#[sqlx::test(fixtures("with_test_user"))]
async fn test_announce_unknown_torrent(pool: PgPool) {
    let service = common::create_test_app(pool, OpenSignups::Enabled).await;

    let req = test::TestRequest::get()
        .uri(concat!(
            "/announce/d2037c66dd3e13044e0d2f9b891c3837?",
            "info_hash=%7C%B3%C6y%9A%FFm%5C%3B%10%A6S%1FF%07%D9%C9%0E%C0%A7&",
            "peer_id=-lt0F01-%3D%91%BB%AC%5C%C69%C0%EDmux&",
            "key=1ab4e687&",
            "compact=1&",
            "port=6968&",
            "uploaded=0&",
            "downloaded=0&",
            "left=14&",
            "event=started"
        ))
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .to_request();

    let resp = test::call_service(&service, req).await;

    // Should fail because there is no torrent matching infohash.
    assert!(
        resp.status().is_client_error(),
        "status {} is not client error",
        resp.status()
    );

    // Any error is okay, as long as it has "failure reason" populated.
    common::read_body_bencode::<WrappedError, _>(resp)
        .await
        .expect("expected failure message");
}

#[sqlx::test(fixtures(
    "with_test_user",
    "with_test_title_group",
    "with_test_edition_group",
    "with_test_torrent"
))]
async fn test_announce_known_torrent(pool: PgPool) {
    let service = common::create_test_app(pool, OpenSignups::Enabled).await;
    let req = test::TestRequest::get()
        .uri(concat!(
            "/announce/d2037c66dd3e13044e0d2f9b891c3837?",
            "info_hash=%11%223DUfw%88%99%AA%BB%CC%DD%EE%FF%00%11%223D&",
            "peer_id=-lt0F01-%3D%91%BB%AC%5C%C69%C0%EDmux&",
            "key=1ab4e687&",
            "compact=1&",
            "port=6968&",
            "uploaded=0&",
            "downloaded=0&",
            "left=14&",
            "event=started"
        ))
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .to_request();

    let resp = test::call_service(&service, req).await;

    // Should succeed because there is both a matching user and info hash.
    assert!(
        resp.status().is_success(),
        "status {} is not success",
        resp.status()
    );

    let resp = common::read_body_bencode::<announce::AnnounceResponse, _>(resp)
        .await
        .expect("could not deserialize announce response");

    // There are no peers, so should be empty.
    assert!(resp.peers.is_empty());
}

#[sqlx::test(fixtures(
    "with_test_user",
    "with_test_title_group",
    "with_test_edition_group",
    "with_test_torrent",
    "with_test_user2",
    "with_test_peers"
))]
async fn test_announce_known_torrent_with_peers(pool: PgPool) {
    let service = common::create_test_app(pool, OpenSignups::Enabled).await;
    let req = test::TestRequest::get()
        .uri(concat!(
            "/announce/d2037c66dd3e13044e0d2f9b891c3837?",
            "info_hash=%11%223DUfw%88%99%AA%BB%CC%DD%EE%FF%00%11%223D&",
            "peer_id=-lt0F01-%3D%91%BB%AC%5C%C69%C0%EDmux&",
            "key=1ab4e687&",
            "compact=1&",
            "port=6968&",
            "uploaded=0&",
            "downloaded=0&",
            "left=14&",
            "event=started"
        ))
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .to_request();

    let resp = test::call_service(&service, req).await;

    // Should succeed because there is both a matching user and info hash.
    assert!(
        resp.status().is_success(),
        "status {} is not success",
        resp.status()
    );

    let resp = common::read_body_bencode::<announce::AnnounceResponse, _>(resp)
        .await
        .expect("could not deserialize announce response");

    // Fixture sets up two non-self peers.
    assert!(resp.peers.len() == 2);

    for announce::PeerCompact { ip, port } in &resp.peers {
        assert_ne!(
            (ip, port),
            (&std::net::Ipv4Addr::new(10, 10, 4, 88), &6968),
            "announce response contains self in peer list"
        );

        assert_ne!(
            (ip, port),
            (&std::net::Ipv4Addr::new(10, 10, 4, 91), &26),
            "peer by the same user is included in peer list"
        );
    }
}
