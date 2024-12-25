use sqlx::mysql::MySqlArguments;
use sqlx::query::{Query, QueryAs};

use crate::models::user::User;

pub fn get_insert_query(
    user: User
) -> Query<'static, sqlx::MySql, MySqlArguments> {
    let query = r#"
        INSERT INTO users (name, email, password)
        VALUES (?, ?, ?);
    "#;

    sqlx::query(query)
        .bind(user.name)
        .bind(user.email)
        .bind(user.password)
}

pub fn get_select_by_id_query(
    id: u64
) -> QueryAs<'static, sqlx::MySql, User, MySqlArguments> {
    let query = r#"
        SELECT * FROM users WHERE id = ?
    "#;

    sqlx::query_as::<_,User>(query)
        .bind(id)
}

pub fn get_select_by_email_query(
    email: String
) -> QueryAs<'static, sqlx::MySql, User, MySqlArguments> {
    let query = r#"
        SELECT * FROM users WHERE email LIKE ?
    "#;
    
    sqlx::query_as::<_,User>(query)
        .bind(email)
}

pub fn get_select_by_name_query(
    name: String
) -> QueryAs<'static, sqlx::MySql, User, MySqlArguments> {
    let query = r#"
        SELECT * FROM users WHERE name LIKE ?
    "#;

    sqlx::query_as::<_,User>(query)
        .bind(name)
}