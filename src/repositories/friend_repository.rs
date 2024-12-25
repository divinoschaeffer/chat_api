use actix_web::web::Data;
use sqlx::MySqlPool;
use crate::models::friendship::Friendship;
use crate::repositories::query_factories::friendship_query_factory::{get_insert_query, get_select_query};

pub async fn create(
    pool: &Data<MySqlPool>,
    friend: Friendship
) -> Result<Friendship, sqlx::Error> {
    let query = get_insert_query(
        friend.first_user_id,
        friend.second_user_id
    );
    let result = query.execute(pool.as_ref()).await?;
    let query = get_select_query(result.last_insert_id() as i64);
    let friend = query.fetch_one(pool.as_ref()).await?;
    Ok(friend)
}