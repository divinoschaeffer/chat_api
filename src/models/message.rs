use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Message {
    pub id: Option<i64>,
    pub discussion_id: Option<i64>,
    pub sender_id: i64,
    pub text: String,
    pub date_created: NaiveDateTime
}