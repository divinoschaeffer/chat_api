use actix_web::web::Data;
use sqlx::MySqlPool;
use crate::models::user::User;
use crate::repositories::query_factories::user_query_factory::{get_insert_query, get_select_by_email_query, get_select_by_id_query, get_select_by_name_query};

pub async fn create(
    pool: &Data<MySqlPool>,
    user: User
) -> Result<User, sqlx::Error> {

    let query = get_insert_query(user.clone());
    let result = query.execute(pool.as_ref()).await?;
    
    let query = get_select_by_id_query(result.last_insert_id());
    let user = query.fetch_one(pool.as_ref()).await?;

    Ok(user)
}

pub async fn get_by_email(
    pool: &Data<MySqlPool>,
    email: String
) -> Result<Option<User>, sqlx::Error> {
    
    let query = get_select_by_email_query(email);
    query.fetch_optional(pool.as_ref()).await
}

pub async fn get_by_name(
    pool: &Data<MySqlPool>,
    name: String
) -> Result<Option<User>, sqlx::Error> {
    
    let query = get_select_by_name_query(name);
    query.fetch_optional(pool.as_ref()).await
}