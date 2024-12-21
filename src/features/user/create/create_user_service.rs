use actix_web::web::Data;
use sqlx::MySqlPool;
use crate::features::user::create::create_user_request::CreateUserRequest;
use crate::models::user::User;
use crate::repositories::user_repository::create;

pub async fn handle(
    pool: Data<MySqlPool>,
    request: CreateUserRequest
) -> Result<User, actix_web::Error> {
    
    let user = User {
        id: None,
        name: request.name,
        email: request.email,
        password: request.password,
    };

    match create(&pool, user).await {
        Ok(created_user) => Ok(created_user),
        Err(e) => {
            log::error!("Error creating user: {}", e);
            Err(actix_web::error::ErrorInternalServerError("Error creating user"))
        }
    }
}
