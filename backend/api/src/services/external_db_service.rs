use actix_web::HttpResponse;
use arcadia_common::error::Result;
use arcadia_storage::connection_pool::ConnectionPool;

use crate::handlers::scrapers::ExternalDBData;

pub async fn check_if_existing_title_group_with_link_exists(
    pool: &ConnectionPool,
    url: &str,
) -> Result<Option<HttpResponse>> {
    let existing_title_group_id = pool.does_title_group_with_link_exist(url).await?;
    if existing_title_group_id.is_some() {
        return Ok(Some(HttpResponse::Ok().json(ExternalDBData {
            title_group: None,
            edition_group: None,
            affiliated_artists: vec![],
            existing_title_group_id,
        })));
    }
    Ok(None)
}
