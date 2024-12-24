use actix_web::body::MessageBody;
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::{Error, HttpMessage};
use actix_web::error::ErrorUnauthorized;
use actix_web::middleware::Next;
use actix_web::web::BytesMut;
use futures_util::StreamExt;
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
struct PayloadData {
    sender_id: i64,
}

pub async fn sender_middleware(
    mut req: ServiceRequest,
    next: Next<impl MessageBody>
) -> Result<ServiceResponse<impl MessageBody>, Error> {

    let stored_user_id = req.extensions().get::<i64>().copied();

    let mut body = BytesMut::new();
    let mut payload = req.take_payload();
    let payload_copy = req.take_payload();
    
    while let Some(chunk) = payload.next().await {
        body.extend_from_slice(&chunk?);
    }
    
    let payload_data: PayloadData = serde_json::from_slice(&body)
        .map_err(|_| ErrorUnauthorized(json!({
            "success": "false",
            "message": "Access unauthorized"
        })))?;

    // Vérifier les IDs
    if stored_user_id != Option::from(payload_data.sender_id) {
        return Err(ErrorUnauthorized(json!({
            "success": "false",
            "message": "User id mismatch"
        })));
    }

    req.set_payload(payload_copy);

    next.call(req).await
}