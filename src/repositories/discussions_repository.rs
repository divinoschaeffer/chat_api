use actix_web::web;
use sqlx::MySqlPool;
use crate::models::discussion::Discussion;
use crate::repositories::query_factories::discussion_query_factory as discussion_query;

pub async fn create(
    pool: &web::Data<MySqlPool>,
    discussion: Discussion
) -> Result<Discussion, sqlx::Error> {
    let mut tx = pool.begin().await?;
    let query = discussion_query::get_insert_query(discussion.created_by);
    let id = query.execute(&mut *tx).await?.last_insert_id();
    discussion_query::
}