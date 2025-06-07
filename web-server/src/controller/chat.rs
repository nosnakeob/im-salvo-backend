use crate::ApiResponse;
use anyhow::Result;
use api_response::prelude::*;
use deadpool_redis::Pool;
use futures_util::StreamExt;
use redis::{AsyncCommands, Client};
use salvo::prelude::*;
use salvo::sse::{SseEvent, SseKeepAlive};

#[endpoint]
pub async fn chat_send(req: &mut Request, depot: &Depot) -> ApiResponse<()> {
    let mut conn = depot.obtain::<Pool>().unwrap().get().await.unwrap();

    let _: () = conn
        .publish(
            "global_room",
            str::from_utf8(req.payload().await.unwrap()).unwrap(),
        )
        .await
        .unwrap();

    ().api_response_without_meta()
}

#[endpoint]
pub async fn user_connected(res: &mut Response, depot: &Depot) -> Result<()> {
    let mut ps = depot.obtain::<Client>().unwrap().get_async_pubsub().await?;

    let (mut sink, stream) = ps.split();

    sink.subscribe("global_room").await?;

    let sse_stream = stream.map(|msg| {
        Ok::<_, salvo::Error>(SseEvent::default().text(msg.get_payload::<String>().unwrap()))
    });

    SseKeepAlive::new(sse_stream).stream(res);

    Ok(())
}
