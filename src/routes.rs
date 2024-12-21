use actix_web::{HttpResponse, Responder, Scope, web};
use actix_web::web::scope;
use crate::features::user::create::create_user_controller::create_user_controller;

async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello Guys")
}

pub fn routes() -> Scope {
    scope("/api")
        .route("/hello-world", web::get().to(hello))
        .route("/register", web::post().to(create_user_controller))
}
