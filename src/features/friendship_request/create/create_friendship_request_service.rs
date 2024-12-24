use actix_web::web::Data;
use serde_json::json;
use sqlx::MySqlPool;
use crate::features::friendship_request::create::create_friendship_request::CreateFriendshipRequest;
use crate::models::friendship_request::FriendshipRequest;
use crate::repositories::friendship_request_repository::{create, get};

pub async fn handle(
    pool: Data<MySqlPool>,
    request: CreateFriendshipRequest
) -> Result<FriendshipRequest, actix_web::Error> {
    
    let friendship_request = FriendshipRequest {
        id: None,
        sender_id: request.sender_id,
        receiver_id: request.receiver_id,
    };
    
    match get(&pool, friendship_request.clone()).await { 
        Ok(option_friendship_req) => {
            if let Some(_) = option_friendship_req {
                log::error!("Friendship request already exist");
                return Err(actix_web::error::ErrorBadRequest(json!({
                    "success": "false",
                    "message": "Friendship request already exist"
                })));
            }
        },
        Err(e) => {
            log::error!("Error getting friendship request: {}", e);
            return Err(actix_web::error::ErrorInternalServerError(json!({
                "success": "false",
                "message": "Error getting friendship request"
            })));
        }
    }
    
    match create(&pool, friendship_request).await { 
        Ok(friendship_request) => Ok(friendship_request),
        Err(e) => {
            log::error!("Error creating friendship request: {}", e);
            Err(actix_web::error::ErrorInternalServerError(json!({
                "success": "false",
                "message": "Error creating friendship request"
            })))
        }
    }
}