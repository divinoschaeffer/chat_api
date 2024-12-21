use actix_identity::Identity;
use actix_web::{HttpMessage, HttpRequest, HttpResponse, Responder};
use actix_web::web::{Data, Json};
use sqlx::MySqlPool;
use crate::features::auth::login::login_controller_request_factory::create_from_payload;
use crate::features::auth::login::login_controller_service::handle;
use crate::payloads::user_payload::UserPayload;

pub async fn login_controller(
    pool: Data<MySqlPool>,
    payload: Json<UserPayload>,
    http_request: HttpRequest
) -> impl Responder {
    match create_from_payload(payload) {
        Ok(request) => {
            match handle(pool, request).await {
                Ok(user) => {
                    Identity::login(
                        &http_request.extensions(), format!("{}", user.id.unwrap())
                    ).unwrap();
                    HttpResponse::Ok().json(user)
                },
                Err(_) => {
                    HttpResponse::InternalServerError().json(
                        serde_json::json!({
                            "error": "Internal Server Error",
                            "message": "Unable to log"
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