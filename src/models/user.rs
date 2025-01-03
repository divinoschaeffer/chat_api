use serde::{Deserialize, Serialize, Serializer};
use serde::ser::SerializeStruct;
#[derive(Debug, sqlx::FromRow, Clone, Deserialize)]
pub struct User {
    #[sqlx(try_from = "i64")]
    pub id: Option<i64>,
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserDTO {
    pub id: Option<i64>,
    pub name: String,
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
