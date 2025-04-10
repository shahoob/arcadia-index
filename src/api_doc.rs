use utoipa::OpenApi;

use crate::{
    handlers::{artist_handler::GetArtistPublicationsQuery, auth_handler::RegisterQuery},
    models::user::{Login, Register},
};

#[derive(OpenApi)]
#[openapi(
    info(title = "arcadia-index API",),
    paths(
        crate::handlers::auth_handler::register,
        crate::handlers::auth_handler::login,
        crate::handlers::artist_handler::get_artist_publications,
    ),
    components(schemas(Register, RegisterQuery, Login, GetArtistPublicationsQuery),)
)]
pub struct ApiDoc;
