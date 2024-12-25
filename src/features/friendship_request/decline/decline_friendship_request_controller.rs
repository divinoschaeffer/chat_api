use actix_web::{HttpRequest, HttpResponse, Responder, web};
use actix_web::web::Data;
use sqlx::MySqlPool;
use crate::features::friendship_request::decline::decline_friendship_request_request::DeclineFriendshipRequest;
use crate::features::friendship_request::decline::decline_friendship_request_service::handle;

pub async fn decline_friendship_request_controller(
    req: HttpRequest,
    pool: Data<MySqlPool>,
    path: web::Path<i64>
) -> impl Responder {
    let id = path.into_inner();
    match handle(req, pool, DeclineFriendshipRequest{id}).await {
        Ok(_) => {
            HttpResponse::Ok().json(serde_json::json!({
                "success": true,
                "message": "friendship request declined"
            }))
        },
        Err(e) => {
            e.error_response()
        }
    }
}