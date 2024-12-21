use actix_web::web::Data;
use argon2::{Argon2, PasswordHash, PasswordHasher};
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use sqlx::MySqlPool;
use crate::features::user::create::create_user_request::CreateUserRequest;
use crate::models::user::User;
use crate::repositories::user_repository::create;

pub async fn handle(
    pool: Data<MySqlPool>,
    request: CreateUserRequest
) -> Result<User, actix_web::Error> {

    let password = hash_password(request.password).map_err(|e| {
        log::error!("Error hashing password: {}", e);
        actix_web::error::ErrorInternalServerError("Error hashing password")
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
            Err(actix_web::error::ErrorInternalServerError("Error creating user"))
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
