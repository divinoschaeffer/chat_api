use actix_web::web::Data;
use argon2::{Argon2, PasswordHash, PasswordVerifier};
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
            let user_password = PasswordHash::new(&user.password).unwrap();
            match Argon2::default().verify_password(request.password.as_bytes(), &user_password) {
                Ok(_) => Ok(user),
                Err(_) => Err(actix_web::error::ErrorBadRequest("Wrong password"))
            }
        },
        Err(e) => {
            log::error!("Error logging: {}", e);
            Err(actix_web::error::ErrorInternalServerError("Error logging"))
        }       
    }
}