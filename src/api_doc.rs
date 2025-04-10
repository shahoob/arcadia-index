use utoipa::OpenApi;

use crate::{
    handlers::auth_handler::RegisterQuery,
    models::user::{Login, Register},
};

#[derive(OpenApi)]
#[openapi(
    info(title = "arcadia-index API",),
    paths(
        crate::handlers::auth_handler::register,
        crate::handlers::auth_handler::login,
    ),
    components(schemas(Register, RegisterQuery, Login),)
)]
pub struct ApiDoc;
