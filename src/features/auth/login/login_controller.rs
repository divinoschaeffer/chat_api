use actix_web::{HttpResponse, Responder};
use actix_web::web::{Data, Json};
use serde_json::json;
use sqlx::MySqlPool;

use crate::features::auth::login::login_controller_request_factory::create_from_payload;
use crate::features::auth::login::login_controller_service::handle;
use crate::payloads::user_payload::UserPayload;
use crate::token::generate_token;

pub async fn login_controller(
    pool: Data<MySqlPool>,
    payload: Json<UserPayload>,
) -> impl Responder {
    match create_from_payload(payload) {
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
                        json!({
                            "error": "Internal Server Error",
                            "message": "Unable to log"
                        })
                    )
                }
            }
        },
        Err(payload_error) => {
            HttpResponse::BadRequest().json(
                json!({
                    "error": "Invalid request",
                    "message": payload_error.to_string()
                })
            )
        }
    }
}