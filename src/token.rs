use std::env;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub user_id: i64,
    pub exp: usize
}

pub fn generate_token(
    user_id: i64
) -> String {

    let private_key = env::var("SECRET_KEY")
        .expect("SECRET_KEY is not set in .env");
    
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::hours(24))
        .expect("Error configuring token")
        .timestamp() as usize;

    let claims = Claims {
        user_id,
        exp: expiration
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(private_key.as_bytes()),
    ).unwrap()
}