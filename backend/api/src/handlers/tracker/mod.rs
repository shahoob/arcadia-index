pub mod get_users;

use actix_web::{
    web::{get, resource, ServiceConfig},
    HttpResponse,
};
use arcadia_common::error::Result;
use arcadia_storage::redis::RedisPoolInterface;
use bincode::config;

pub fn config<R: RedisPoolInterface + 'static>(cfg: &mut ServiceConfig) {
    cfg.service(resource("/users").route(get().to(self::get_users::exec::<R>)));
}

fn binary_response<T: bincode::Encode>(value: &T) -> Result<HttpResponse> {
    let config = config::standard();
    let bytes = bincode::encode_to_vec(value, config).expect("error encoding to bincode");
    Ok(HttpResponse::Ok()
        .content_type("application/octet-stream")
        .body(bytes))
}
