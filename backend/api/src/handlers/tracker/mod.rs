pub mod get_env;
pub mod get_infohash_2_id;
pub mod get_passkey_2_id;
pub mod get_torrents;
pub mod get_users;
pub mod post_user_updates;

use actix_web::{
    web::{get, post, resource, ServiceConfig},
    HttpResponse,
};
use arcadia_common::error::Result;
use arcadia_storage::redis::RedisPoolInterface;
use bincode::config;

// TODO: protect by only allowing requests from tracker's ip
pub fn config<R: RedisPoolInterface + 'static>(cfg: &mut ServiceConfig) {
    cfg.service(resource("/env").route(get().to(self::get_env::exec::<R>)));
    cfg.service(resource("/users").route(get().to(self::get_users::exec::<R>)));
    cfg.service(resource("/torrents").route(get().to(self::get_torrents::exec::<R>)));
    cfg.service(resource("/passkeys-2-ids").route(get().to(self::get_passkey_2_id::exec::<R>)));
    cfg.service(resource("/infohashes-2-ids").route(get().to(self::get_infohash_2_id::exec::<R>)));
    cfg.service(resource("/user-updates").route(post().to(self::post_user_updates::exec::<R>)));
}

fn binary_response<T: bincode::Encode>(value: &T) -> Result<HttpResponse> {
    let config = config::standard();
    let bytes = bincode::encode_to_vec(value, config).expect("error encoding to bincode");

    Ok(HttpResponse::Ok()
        .content_type("application/octet-stream")
        .body(bytes))
}
