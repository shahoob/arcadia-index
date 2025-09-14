pub mod create_collage;
pub mod create_collage_entry;

use actix_web::web::{post, resource, ServiceConfig};
use arcadia_storage::redis::RedisPoolInterface;

pub fn config<R: RedisPoolInterface + 'static>(cfg: &mut ServiceConfig) {
    cfg.service(resource("").route(post().to(self::create_collage::exec::<R>)));
    cfg.service(resource("/entries").route(post().to(self::create_collage_entry::exec::<R>)));
}
