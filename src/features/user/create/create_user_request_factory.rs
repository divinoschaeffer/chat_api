use actix_web::web;

use crate::features::user::create::create_user_request::CreateUserRequest;
use validator::Validate;
use crate::payloads::create_user_payload::CreateUserPayload;

pub fn create_from_payload(
    payload: web::Json<CreateUserPayload>
) -> Result<CreateUserRequest, actix_web::Error> {
    
    if let Err(validation_errors) = payload.validate() {
        return Err(actix_web::error::ErrorBadRequest(validation_errors));
    }

    Ok(CreateUserRequest {
        name: payload.name.clone(),
        email: payload.email.clone(),
        password: payload.password.clone()
    })
}