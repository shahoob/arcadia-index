mod handlers;
mod models;
mod repositories;
mod routes;

use actix_cors::Cors;
use actix_web::{App, HttpServer, web::Data};
use dotenv;
use routes::init;
use sqlx::postgres::PgPoolOptions;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::from_filename(".env.local").ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Error building a connection pool");

    let host = env::var("ACTIX_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("ACTIX_PORT").unwrap_or_else(|_| "8080".to_string());
    println!("Server running at http://{}:{}", host, port);

    HttpServer::new(move || {
        let cors = Cors::permissive();
        App::new()
            .wrap(cors)
            .app_data(Data::new(pool.clone()))
            .configure(init) // Initialize routes
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}
