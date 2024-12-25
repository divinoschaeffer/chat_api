use actix_web::web::Data;
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use serde_json::json;
use sqlx::MySqlPool;
use crate::features::auth::login::login_controller_request::LoginRequest;
use crate::models::user::User;
use crate::repositories::user_repository::get_by_email;

pub async fn handle(
    pool: Data<MySqlPool>,
    request: LoginRequest
) -> Result<User, actix_web::Error> {
    
    match get_by_email(&pool, request.email).await {
        Ok(user) => {
            if let Some(user) = user {
                let user_password = PasswordHash::new(&user.password).unwrap();
                match Argon2::default().verify_password(request.password.as_bytes(), &user_password) {
                    Ok(_) => return Ok(user),
                    Err(_) => Err(actix_web::error::ErrorBadRequest(json!({
                        "success": "false",
                        "message": "Wrong password"
                    })))
                }
            } else {
                log::error!("User not found.");
                Err(actix_web::error::ErrorBadRequest(json!({
                    "success": "false",
                    "message": "User with this email not found"
                })))
            }
        },
        Err(e) => {
            log::error!("Error logging: {}", e);
            Err(actix_web::error::ErrorInternalServerError(json!({
                "success": "false",
                "message": "Error logging user"
            })))
        }       
    }
}