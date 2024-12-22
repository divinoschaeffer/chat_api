use actix_web::{HttpResponse, Responder, Scope, web};
use actix_web::web::scope;

use crate::features::auth::login::login_controller::login_controller;
use crate::features::user::create::create_user_controller::create_user_controller;
use crate::middlewares::auth_middleware::Auth;

async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello Guys")
}

pub fn routes() -> Scope {
    scope("/api")
        .service(web::resource("/register").route(web::post().to(create_user_controller)))
        .service(web::resource("/login").route(web::post().to(login_controller)))
        .service(
            scope("")
                .wrap(Auth)
                .service(web::resource("/hello-world").route(web::get().to(hello)))
        )
}
