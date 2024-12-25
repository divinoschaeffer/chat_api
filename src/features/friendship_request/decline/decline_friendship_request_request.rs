use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub struct DeclineFriendshipRequest {
    pub id: i64
}