use sqlx::mysql::MySqlArguments;
use sqlx::query::{Query, QueryAs};
use crate::models::friendship_request::FriendshipRequest;

pub fn get_insert_query(
    sender_id: i64,
    receiver_id: i64,
) -> Query<'static, sqlx::MySql, MySqlArguments> {
    let query = r#"
        INSERT INTO friendship_requests (sender_id, receiver_id)
        VALUES (?, ?);
    "#;
    
    sqlx::query(query)
        .bind(sender_id)
        .bind(receiver_id)
}

pub fn get_select_query(
    sender_id: i64,
    receiver_id: i64,
) -> QueryAs<'static, sqlx::MySql, FriendshipRequest, MySqlArguments> {
    let query = r#"
        SELECT * FROM friendship_requests
        WHERE sender_id = ? AND receiver_id = ?
    "#;
    
    sqlx::query_as::<_,FriendshipRequest>(query)
        .bind(sender_id)
        .bind(receiver_id)
}

pub fn get_select_by_id_query(
    id: i64,
) -> QueryAs<'static, sqlx::MySql, FriendshipRequest, MySqlArguments> {
    let query = r#"
        SELECT * FROM friendship_requests
        WHERE id = ?
    "#;
    
    sqlx::query_as::<_,FriendshipRequest>(query)
        .bind(id)
}

pub fn get_delete_query(
    id: i64
) -> Query<'static, sqlx::MySql, MySqlArguments> {
    let query = r#"
        DELETE FROM friendship_requests
        WHERE id = ?
    "#;
    
    sqlx::query(query)
        .bind(id)
}