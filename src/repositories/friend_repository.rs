use actix_web::web::Data;
use sqlx::MySqlPool;
use crate::models::friendship::Friendship;
use crate::repositories::query_factories::friendship_query_factory::{get_insert_query, get_select_query};
use crate::repositories::query_factories::friendship_request_query_factory::get_delete_query;

pub async fn create(
    pool: &Data<MySqlPool>,
    friend: Friendship
) -> Result<Friendship, sqlx::Error> {
    let mut tx = pool.begin().await?;
    if friend.friend_request_id.is_some() {
        let query = get_delete_query(friend.friend_request_id.unwrap());
        query.execute(&mut *tx).await?;
    }
    let query = get_insert_query(
        friend.first_user_id,
        friend.second_user_id
    );
    let result = query.execute(&mut *tx).await?;
    let query = get_select_query(result.last_insert_id() as i64);
    let friend = query.fetch_one(&mut *tx).await?;
    tx.commit().await?;
    Ok(friend)
}