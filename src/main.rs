mod features;
mod models;
mod routes;
mod repositories;
mod payloads;
mod middlewares;
mod token;

use std::env;
use actix_web::{App, HttpServer, middleware, web};
use sqlx::MySqlPool;
use crate::routes::routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();

    env_logger::init_from_env(
        env_logger::Env::new()
            .default_filter_or("info")
    );

    log::info!("setting up app from environment");

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env");

    log::info!("initializing database connection");

    let pool = MySqlPool::connect(db_url.as_str())
        .await
        .expect("Failed to create pool");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(routes())
            .wrap(middleware::NormalizePath::trim())
            .wrap(middleware::Logger::default())
    })
        .bind(("127.0.0.1", 8080))?
        .workers(2)
        .run()
        .await
}
