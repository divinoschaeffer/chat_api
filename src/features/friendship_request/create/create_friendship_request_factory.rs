use actix_web::web;
use crate::features::friendship_request::create::create_friendship_request::CreateFriendshipRequest;
use crate::payloads::create_friendship_request_payload::CreateFriendshipRequestPayload;
use validator::Validate;

pub fn create_from_payload(
    payload: web::Json<CreateFriendshipRequestPayload>
) -> Result<CreateFriendshipRequest, actix_web::Error> {
    if let Err(validation_errors) = payload.validate() {
        return Err(actix_web::error::ErrorBadRequest(validation_errors));
    }
    
    Ok(CreateFriendshipRequest{
        sender_id: payload.sender_id,
        receiver_id: payload.receiver_id
    })
}