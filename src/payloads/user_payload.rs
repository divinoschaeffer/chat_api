use serde::Deserialize;
use validator::Validate;

#[derive(Validate, Deserialize)]
pub struct UserPayload {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
    pub password: String
}