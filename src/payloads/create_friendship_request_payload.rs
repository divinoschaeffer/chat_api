use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct CreateFriendshipRequestPayload {
    pub sender_id: i64,
    pub receiver_id: i64,
}