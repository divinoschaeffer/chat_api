use sqlx::mysql::MySqlArguments;
use sqlx::query::{Query, QueryAs};

use crate::models::friendship::Friendship;

pub fn get_insert_query(
    first_user_id: i64,
    second_user_id: i64
) -> Query<'static, sqlx::MySql, MySqlArguments> {
    let query = r#"
        INSERT INTO friendships (first_user_id, second_user_id)
        VALUES (?, ?)
    "#;
    
    sqlx::query(query)
        .bind(first_user_id)
        .bind(second_user_id)
}

pub fn get_select_query(
    id: i64
) -> QueryAs<'static, sqlx::MySql, Friendship, MySqlArguments> {
    let query = r#"
        SELECT * FROM friendships
        WHERE id = ?
    "#;

    sqlx::query_as::<_, Friendship>(query)
        .bind(id)
}

pub fn get_select_by_user_id(
    id: i64
) -> Query<'static, sqlx::MySql, MySqlArguments> {
    let query = r#"
        SELECT * FROM friendships
        WHERE first_user_id = ? OR second_user_id = ?
    "#;

    sqlx::query(query)
        .bind(id)
        .bind(id)
}

pub fn get_delete_query(
    id: i64
) -> Query<'static, sqlx::MySql, MySqlArguments> {
    let query = r#"
        DELETE FROM friendships
        WHERE id = ?
    "#;

    sqlx::query(query)
        .bind(id)
}