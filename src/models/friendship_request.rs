use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow, Clone)]
pub struct FriendshipRequest {
    pub id: Option<i64>,
    pub sender_id: i64,
    pub receiver_id: i64
}