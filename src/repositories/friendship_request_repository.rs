use actix_web::web::Data;
use sqlx::MySqlPool;
use crate::models::friendship_request::FriendshipRequest;
use crate::repositories::query_factories::friendship_request_query_factory::{get_delete_query, get_insert_query, get_select_by_id_query, get_select_query};

pub async fn create(
    pool: &Data<MySqlPool>,
    friendship_request: FriendshipRequest
) -> Result<FriendshipRequest, sqlx::Error> {
    let query = get_insert_query(
        friendship_request.sender_id,
        friendship_request.receiver_id
    );
    let result = query.execute(pool.as_ref()).await?;
    
    let query = get_select_by_id_query(result.last_insert_id() as i64);
    let request = query.fetch_one(pool.as_ref()).await?;
    
    Ok(request)
}

pub async fn get_by_id(
    pool: &Data<MySqlPool>,
    id: i64
) -> Result<Option<FriendshipRequest>, sqlx::Error> {
    let query = get_select_by_id_query(id);
    query.fetch_optional(pool.as_ref()).await
}

pub async fn get(
    pool: &Data<MySqlPool>,
    friendship_request: FriendshipRequest
) -> Result<Option<FriendshipRequest>, sqlx::Error> {
    let query = get_select_query(
        friendship_request.sender_id,
        friendship_request.receiver_id
    );
    query.fetch_optional(pool.as_ref()).await
}

pub async fn delete(
    pool: &Data<MySqlPool>,
    id: i64
) -> Result<(), sqlx::Error> {
    let query = get_delete_query(id);
    query.execute(pool.as_ref()).await?;
    
    Ok(())
}