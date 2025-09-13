pub mod create_collage;

use actix_web::web::{post, resource, ServiceConfig};
use arcadia_storage::redis::RedisPoolInterface;

pub fn config<R: RedisPoolInterface + 'static>(cfg: &mut ServiceConfig) {
    cfg.service(resource("").route(post().to(self::create_collage::exec::<R>)));
    // cfg.service(resource("/vote").route(post().to(self::create_torrent_request_vote::exec::<R>)));
}
