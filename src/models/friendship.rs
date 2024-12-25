use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Clone, Serialize, Deserialize, Debug, FromRow)]
pub struct Friendship {
    pub id: Option<i64>,
    #[sqlx(default)]
    pub friend_request_id: Option<i64>,
    pub first_user_id: i64,
    pub second_user_id: i64,
}