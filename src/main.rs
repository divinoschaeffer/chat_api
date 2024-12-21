mod features;
mod models;
mod routes;
mod repositories;
mod payloads;

use std::env;
use actix_web::{App, HttpServer, middleware, web};
use actix_identity::{IdentityMiddleware};
use actix_session::config::PersistentSession;
use actix_session::SessionMiddleware;
use actix_session::storage::CookieSessionStore;
use actix_web::cookie::Key;
use actix_web::cookie::time::Duration;
use sqlx::MySqlPool;
use crate::routes::routes;

const SESSION_DURATION: Duration = Duration::days(1);

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();

    env_logger::init_from_env(
        env_logger::Env::new()
            .default_filter_or("info")
    );

    log::info!("setting up app from environment");

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env");
    let private_key = env::var("SECRET_KEY").expect("SECRET_KEY is not set in .env");

    let secret_key = Key::from(&*base64::decode(&private_key).unwrap());

    log::info!("initializing database connection");

    let pool = MySqlPool::connect(db_url.as_str())
        .await
        .expect("Failed to create pool");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(routes())
            .wrap(IdentityMiddleware::default())
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), secret_key.clone())
                    .cookie_name("authentication".to_string())
                    .cookie_secure(false) // TODO: Change in prod
                    .session_lifecycle(PersistentSession::default().session_ttl(SESSION_DURATION))
                    .build()
            )
            .wrap(middleware::NormalizePath::trim())
    })
        .bind(("127.0.0.1", 8080))?
        .workers(2)
        .run()
        .await
}
