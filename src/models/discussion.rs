use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use crate::models::message::Message;
use crate::models::user::User;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Discussion {
    pub id: Option<i64>,
    pub created_by: i64,
    #[sqlx(default)]
    pub user: Vec<User>,
    #[sqlx(default)]
    pub messages: Vec<Message>,
    pub date_created: Option<NaiveDateTime>
}