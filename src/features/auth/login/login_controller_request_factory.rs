use actix_web::web;
use crate::features::auth::login::login_controller_request::LoginRequest;
use crate::payloads::user_payload::UserPayload;
use validator::Validate;

pub fn create_from_payload(
    payload: web::Json<UserPayload>
) -> Result<LoginRequest, actix_web::Error> {
    if let Err(validation_errors) = payload.validate() {
        return Err(actix_web::error::ErrorBadRequest(validation_errors));
    }

    Ok(LoginRequest {
        email: payload.email.clone(),
        password: payload.password.clone()
    })
}