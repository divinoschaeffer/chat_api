use serde::Deserialize;
use validator::Validate;

#[derive(Validate, Deserialize)]
pub struct CreateUserPayload {
    #[validate(length(min = 1, message = "Name must not be empty"))]
    pub name: String,
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
    #[validate(length(min = 8, message = "Password must be at least 8 characters long"))]
    pub password: String,
}