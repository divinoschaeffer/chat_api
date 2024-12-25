use actix_http::HttpMessage;
use actix_web::error::{ErrorBadRequest, ErrorInternalServerError, ErrorUnauthorized};
use actix_web::HttpRequest;
use actix_web::web::Data;
use serde_json::json;
use sqlx::MySqlPool;
use crate::features::friendship_request::accept::accept_friendship_request_request::AcceptFriendshipRequestRequest;
use crate::models::friendship::Friendship;
use crate::repositories::friend_repository::create;
use crate::repositories::friendship_request_repository::{get_by_id};

pub async fn handle(
    req: HttpRequest,
    pool: Data<MySqlPool>,
    request: AcceptFriendshipRequestRequest
) -> Result<Friendship, actix_web::Error> {

    let stored_user_id = req.extensions()
        .get::<i64>()
        .copied()
        .unwrap();

    println!("{}", stored_user_id);
    match get_by_id(&pool, request.id.clone()).await {
        Ok(friend_request) => {
            if let Some(friend_request) = friend_request {
                if friend_request.receiver_id != stored_user_id {
                    return Err(ErrorUnauthorized(json!({
                        "success": "false",
                        "message": "User mismatch"
                    })));
                } else {
                    let friend = Friendship {
                        id: None,
                        friend_request_id: friend_request.id,
                        first_user_id: friend_request.sender_id,
                        second_user_id: friend_request.receiver_id,
                    };

                    match create(&pool, friend).await {
                        Ok(friend) => {
                            Ok(friend)
                        }
                        Err(e) => {
                            log::error!("Error creating friendship: {}", e);
                            Err(actix_web::error::ErrorInternalServerError(json!({
                                "success": "false",
                                "message": "Error creating friendship"
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