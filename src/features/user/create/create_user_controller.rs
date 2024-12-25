use actix_web::{HttpResponse, Responder, web};
use actix_web::web::Data;
use serde_json::json;
use sqlx::MySqlPool;

use crate::features::user::create::create_user_request_factory::create_from_payload;
use crate::features::user::create::create_user_service::handle;
use crate::payloads::create_user_payload::CreateUserPayload;
use crate::token::generate_token;

pub async fn create_user_controller(
    pool: Data<MySqlPool>,
    payload: web::Json<CreateUserPayload>,
) -> impl Responder {

    match create_from_payload(payload){
        Ok(request) => {
            match handle(pool, request).await {
                Ok(user) => {
                    let token = generate_token(user.id.unwrap().clone());
                    HttpResponse::Ok().json(json!({
                        "user": user,
                        "token": token
                    }))
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
