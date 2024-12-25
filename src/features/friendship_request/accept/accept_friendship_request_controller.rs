use actix_web::{HttpRequest, HttpResponse, Responder, web};
use actix_web::web::Data;
use sqlx::MySqlPool;

use crate::features::friendship_request::accept::accept_friendship_request_request::AcceptFriendshipRequestRequest;
use crate::features::friendship_request::accept::accept_friendship_request_service::handle;

pub async fn accept_friendship_request_controller(
    req: HttpRequest,
    pool: Data<MySqlPool>,
    path: web::Path<i64>
) -> impl Responder {
    let id = path.into_inner();
    match handle(req, pool, AcceptFriendshipRequestRequest{id}).await {
        Ok(friendship) => {
            HttpResponse::Ok().json(friendship)
        },
        Err(e) => {
            e.error_response()
        }
    }
}