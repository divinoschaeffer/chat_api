use actix_identity::Identity;
use actix_web::{HttpMessage, HttpRequest, HttpResponse, Responder, web};
use actix_web::web::Data;
use sqlx::MySqlPool;
use crate::features::user::create::create_user_request_factory::create_from_payload;
use crate::features::user::create::create_user_service::handle;
use crate::payloads::create_user_payload::CreateUserPayload;

pub async fn create_user_controller(
    pool: Data<MySqlPool>,
    payload: web::Json<CreateUserPayload>,
    http_request: HttpRequest
) -> impl Responder {

    match create_from_payload(payload){
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
