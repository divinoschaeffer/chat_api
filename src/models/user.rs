use serde::{Deserialize, Serialize, Serializer};
use serde::ser::SerializeStruct;
use validator::Validate;
#[derive(Debug, sqlx::FromRow, Clone)]
pub struct User {
    #[sqlx(try_from = "i64")]
    pub id: Option<i64>,
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Validate, Deserialize)]
pub struct UserPayload{
    #[validate(length(min = 1, message = "Name must not be empty"))]
    pub name: String,
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
    #[validate(length(min = 8, message = "Password must be at least 8 characters long"))]
    pub password: String,
}

impl Serialize for User {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        let mut state = serializer.serialize_struct("User", 3)?;

        state.serialize_field("id", &self.id)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("email", &self.email)?;

        state.end()
    }
}
