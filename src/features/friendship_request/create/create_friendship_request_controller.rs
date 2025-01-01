use actix_web::{HttpRequest, HttpResponse, Responder, web};
use actix_web::web::Data;
use serde_json::json;
use sqlx::MySqlPool;

use crate::features::friendship_request::create::create_friendship_request_factory::create_from_payload;
use crate::features::friendship_request::create::create_friendship_request_service::handle;
use crate::payloads::create_friendship_request_payload::CreateFriendshipRequestPayload;

pub async fn create_friendship_request_controller(
    req: HttpRequest,
    pool: Data<MySqlPool>,
    payload: web::Json<CreateFriendshipRequestPayload>
) -> impl Responder {
    
    match create_from_payload(payload) {
        Ok(request) => {
            match handle(req, pool, request).await {
                Ok(friend_request) => {
                    HttpResponse::Ok().json(friend_request)
                },
                Err(e) => {
                    e.error_response()
                }
            }
        },
        Err(payload_error) => {
            HttpResponse::BadRequest().json(
                json!({
                        "success": "false",
                        "message": payload_error.to_string()
                    })
            )
        }
    }
}