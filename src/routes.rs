use actix_web::{HttpResponse, Responder, Scope, web};
use actix_web::web::scope;

use crate::features::auth::login::login_controller::login_controller;
use crate::features::friendship_request::accept::accept_friendship_request_controller::accept_friendship_request_controller;
use crate::features::friendship_request::create::create_friendship_request_controller::create_friendship_request_controller;
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
                .service(
                    scope("/friend-request")
                        .service(web::resource("").route(web::post().to(create_friendship_request_controller)))
                        .service(web::resource("/accept/{friendship_request_id}").route(web::post().to(accept_friendship_request_controller)))
                )
        )
}
