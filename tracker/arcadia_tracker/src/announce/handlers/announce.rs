use actix_web::web::Path;

use crate::announce::error::AnnounceError;

#[utoipa::path(
    post,
    operation_id = "Announce",
    tag = "Announce",
    path = "/{passkey}/announce",
    responses(
        (status = 200, description = "Announce"),
    )
)]
pub async fn exec(/*arc: Data<Tracker>, */ passkey: Path<String>) /*-> Result<HttpResponse>*/
{
    let _passkey = u128::from_str_radix(&passkey, 16).map_err(|_| AnnounceError::InvalidPassKey);
    //?;

    // Ok(HttpResponse::Ok())
}
