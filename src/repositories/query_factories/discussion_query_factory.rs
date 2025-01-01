use sqlx::mysql::MySqlArguments;
use sqlx::query::Query;

pub fn get_insert_query(
    created_by: i64
) -> Query<'static, sqlx::MySql, MySqlArguments> {
    let query = r#"
        INSERT INTO discussions (created_by)
        VALUES (?)
    "#;
    
    sqlx::query(query)
        .bind(created_by)
}

pub fn get_select_query_by_id(
    id: i64
) -> Query<'static, sqlx::MySql, MySqlArguments> {
    let query = r#"
        SELECT
        discussions.id AS discussions_id,
        created_by AS discussions_created_by,
        discussions.date_created AS discussions_date_created,
        messages.id AS messages_id,
        sender_id AS messages_sender_id,
        messages.date_created AS messages_date_created,
        users.id AS users_id,
        user_discussion.date_created AS user_discussion_date_created,
        name AS users_name
        FROM discussions
        INNER JOIN messages ON discussions.id = messages.discussion_id
        INNER JOIN user_discussion ON discussions.id = user_discussion.discussion_id
        INNER JOIN users ON users.id = user_discussion.user_id
        WHERE discussions.id = ?
    "#;
    
    sqlx::query(query)
        .bind(id)
}