mod handlers;
mod models;
mod repositories;
mod routes;

use actix_cors::Cors;
use actix_web::{App, HttpServer, middleware, web::Data};
use dotenv;
use routes::init;
use sqlx::postgres::PgPoolOptions;
use std::env;
// use utoipa_actix_web::AppExt;
// use utoipa_swagger_ui::SwaggerUi;

use arcadia_index::{Arcadia, Error, OpenSignups, Result};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::from_filename(".env.local").ok();

    env_logger::init_from_env(env_logger::Env::default().default_filter_or("debug"));

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Error building a connection pool");

    let host = env::var("ACTIX_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("ACTIX_PORT").unwrap_or_else(|_| "8080".to_string());
    println!("Server running at http://{}:{}", host, port);

    let open_signups = if env::var("ARCADIA_OPEN_SIGNUPS").unwrap() == "true" {
        OpenSignups::Enabled
    } else {
        OpenSignups::Disabled
    };

    HttpServer::new(move || {
        let cors = Cors::permissive();
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(cors)
            .app_data(Data::new(Arcadia {
                pool: pool.clone(),
                open_signups,
            }))
            .configure(init) // Initialize routes
        // .into_utoipa_app()
        // .openapi_service(|api| {
        //     SwaggerUi::new("/swagger-ui/{_:.*}").url("/api/openapi.json", api)
        // })
        // .into_app()
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}
