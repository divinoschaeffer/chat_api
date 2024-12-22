use actix_web::web::Data;
use argon2::{Argon2, PasswordHash, PasswordHasher};
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use serde_json::json;
use sqlx::MySqlPool;
use crate::features::user::create::create_user_request::CreateUserRequest;
use crate::models::user::User;
use crate::repositories::user_repository::{create, get_by_email, get_by_name};

pub async fn handle(
    pool: Data<MySqlPool>,
    request: CreateUserRequest
) -> Result<User, actix_web::Error> {
    
    match get_by_email(&pool, request.email.clone()).await {
        Ok(option_user) => {
            if let Some(_) = option_user {
                log::error!("User with email :{} already exist", request.email);
                return Err(actix_web::error::ErrorBadRequest(json!({
                    "success": "false",
                    "message": "User with this email already exist"
                })));
            }
        }
        Err(e) => {
            log::error!("Error getting user by email: {}", e);
            return Err(actix_web::error::ErrorInternalServerError(json!({
                "success": "false",
                "message": "Error getting user by email"
            })));
        }
    }

    match get_by_name(&pool, request.name.clone()).await {
        Ok(option_user) => {
            if let Some(_) = option_user {
                log::error!("User with name :{} already exist", request.name);
                return Err(actix_web::error::ErrorBadRequest(json!({
                "success": "false",
                "message": "User with this name already exist"
            })));
            }
        }
        Err(e) => {
            log::error!("Error getting user by name: {}", e);
            return Err(actix_web::error::ErrorInternalServerError(json!({
                "success": "false",
                "message": "Error getting user by name"
            })));
        }
    }

    let password = hash_password(request.password).map_err(|e| {
        log::error!("Error hashing password: {}", e);
        actix_web::error::ErrorInternalServerError(json!({
                "success": "false",
                "message": "Error hashing password"
            }))
    })?;

    let user = User {
        id: None,
        name: request.name,
        email: request.email,
        password,
    };

    match create(&pool, user).await {
        Ok(created_user) => Ok(created_user),
        Err(e) => {
            log::error!("Error creating user: {}", e);
            Err(actix_web::error::ErrorInternalServerError(json!({
                "success": "false",
                "message": "Error creating user"
            })))
        }
    }
}

fn hash_password(
    password: String
) -> Result<String, argon2::password_hash::Error> {

    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(password.as_bytes(), &salt)?.to_string();
    let parsed_hash = PasswordHash::new(&password_hash)?.to_string();
    Ok(parsed_hash)
}
