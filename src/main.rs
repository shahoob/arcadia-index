mod handlers;
mod models;
mod repositories;
mod routes;

use actix_web::{App, HttpServer, web::Data};
use dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;

use routes::routes::init;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::from_filename(".env.local").ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Error building a connection pool");

    let host = env::var("ACTIX_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("ACTIX_PORT").unwrap_or_else(|_| "8080".to_string());
    println!("Server running at http://{}:{}", host, port);

    HttpServer::new(move || {
        App::new().app_data(Data::new(pool.clone())).configure(init) // Initialize routes
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}
