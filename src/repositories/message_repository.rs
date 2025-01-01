use actix_web::web;
use sqlx::MySqlPool;
use crate::models::message::Message;

pub async fn create(
    pool: &web::Data<MySqlPool>,
    message: Message
) -> Result<Message, sqlx::Error> {
    let discussion_id = None;
    if message.discussion_id.is_none() {
        
    }
}