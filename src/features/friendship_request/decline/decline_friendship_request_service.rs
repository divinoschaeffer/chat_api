use actix_http::HttpMessage;
use actix_web::error::{ErrorBadRequest, ErrorInternalServerError, ErrorUnauthorized};
use actix_web::HttpRequest;
use actix_web::web::Data;
use serde_json::json;
use sqlx::MySqlPool;
use crate::features::friendship_request::decline::decline_friendship_request_request::DeclineFriendshipRequest;
use crate::repositories::friendship_request_repository::{delete, get_by_id};

pub async fn handle(
    req: HttpRequest,
    pool: Data<MySqlPool>,
    request: DeclineFriendshipRequest
) -> Result<(), actix_web::Error> {

    let stored_user_id = req.extensions()
        .get::<i64>()
        .copied()
        .unwrap();

    match get_by_id(&pool, request.id.clone()).await {
        Ok(friend_request) => {
            if let Some(friend_request) = friend_request {
                if friend_request.receiver_id != stored_user_id {
                    return Err(ErrorUnauthorized(json!({
                        "success": "false",
                        "message": "User mismatch"
                    })));
                } else {
                    match delete(&pool, friend_request.id.unwrap()).await {
                        Ok(_) => return Ok(()),
                        Err(e) => {
                            log::error!("Error deleting friendship request: {}", e);
                            Err(ErrorInternalServerError(json!({
                                "success": "false",
                                "message": "Error deleting friendship request"
                            })))
                        }
                    }
                }
            } else {
                return Err(ErrorBadRequest(json!({
                    "success": "false",
                    "message": "Friendship request does not exist"
                })));
            }
        },
        Err(_) => {
            return Err(ErrorInternalServerError(json!({
                "success": "false",
                "message": "InternalServerError"
            })));
        }
    }
}