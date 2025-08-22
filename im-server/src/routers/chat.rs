use crate::ApiResponse;
use crate::models::msg::Message;
use anyhow::Result;
use api_response::prelude::*;
use deadpool_redis::Pool;
use deadpool_redis::redis::AsyncCommands;
use futures_util::StreamExt;
use im_common::jwt::JwtClaims;
use redis::Client;
use salvo::oapi::extract::JsonBody;
use salvo::prelude::*;
use salvo::sse::{SseEvent, SseKeepAlive};

#[endpoint]
pub async fn send(json: JsonBody<Message>, depot: &Depot) -> ApiResponse<()> {
    let id = &depot.jwt_auth_data::<JwtClaims>().unwrap().claims.id;

    let mut msg = json.into_inner();
    msg.sender_id.get_or_insert(id.clone());

    let mut conn = depot.obtain::<Pool>().unwrap().get().await.unwrap();

    let _: () = conn.publish("global_room", msg).await.unwrap();

    ().api_response_without_meta()
}

#[endpoint]
pub async fn connect(res: &mut Response, depot: &Depot) -> Result<()> {
    let ps = depot.obtain::<Client>().unwrap().get_async_pubsub().await?;

    let (mut sink, stream) = ps.split();

    sink.subscribe("global_room").await?;

    let sse_stream = stream.map(|msg| {
        Ok::<_, salvo::Error>(SseEvent::default().text(msg.get_payload::<String>().unwrap()))
    });

    SseKeepAlive::new(sse_stream).stream(res);

    Ok(())
}
