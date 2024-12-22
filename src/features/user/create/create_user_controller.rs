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
                Err(_) => {
                    HttpResponse::InternalServerError().json(
                        serde_json::json!({
                            "error": "Internal Server Error",
                            "message": "Unable to create user"
                        })
                    )
                }
            }
        },
        Err(payload_error) => {
            HttpResponse::BadRequest().json(
                serde_json::json!({
                    "error": "Invalid request",
                    "message": payload_error.to_string()
                })
            )
        }
    }
}
