use actix_web::{
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    middleware::Next,
    web::BytesMut,
    Error, HttpMessage,
};
use futures_util::StreamExt;

use crate::handlers::ReqPayload;

#[allow(dead_code)]
pub async fn event_mw(
    mut req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    // pre-processing
    let mut body = BytesMut::new();
    let mut stream = req.take_payload();

    while let Some(item) = stream.next().await {
        body.extend_from_slice(&item?);
    }

    let req_payload = serde_json::from_slice::<ReqPayload>(&body)?;

    let (_, mut payload) = actix_http::h1::Payload::create(true);
    payload.unread_data(body.into());
    req.set_payload(payload.into());

    // call the service
    let res = next.call(req).await;

    // post-processing

    // TODO: process the response
    log::info!("user id:{}", req_payload.user_id);

    res
}
