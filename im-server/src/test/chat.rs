use std::mem::size_of;
use anyhow::Result;
use deadpool_redis::{Config, Connection};
use redis::streams::{StreamReadOptions, StreamReadReply};
use redis::{AsyncCommands, AsyncConnectionConfig, Commands, PubSubCommands, RedisResult, Value};
use rocket::futures::future::join_all;
use rocket::futures::{stream, SinkExt};
use std::sync::Arc;
use std::time::Duration;
use rocket::futures::channel::oneshot::Cancellation;
use rocket::http::ext::IntoCollection;
use tokio::select;
use tokio::sync::{broadcast, mpsc, oneshot};
use tokio::task::JoinHandle;
use tokio::time::timeout;
use tokio_stream::StreamExt;
use tokio_stream::wrappers::ReceiverStream;
use tokio_util::sync::CancellationToken;

#[tokio::test]
async fn t_channel_pubsub() {
    // 广播通道
    let (tx, _rx) = broadcast::channel(1024);

    let mut tasks = vec![];
    for i in 0..2 {
        let (txi, mut rxi) = (tx.clone(), tx.subscribe());
        let t = tokio::spawn(async move {
            // 发布
            txi.send(format!("t{} msg", i)).unwrap();

            // 订阅, 加超时避免阻塞
            while let Ok(Ok(msg)) = timeout(Duration::from_millis(10), rxi.recv()).await {
                println!("t{}: {}", i, msg);
            }
        });
        tasks.push(t);
    }

    tx.send("hello".to_string()).unwrap();
    tx.send("world".to_string()).unwrap();

    join_all(tasks).await;
}

#[tokio::test]
async fn t_redis_pubsub() -> Result<()> {
    // 无法池化发布订阅资源
    let client = Arc::new(redis::Client::open("redis://127.0.0.1")?);

    let mut tasks = vec![];
    for i in 0..2 {
        let clienti = client.clone();
        let t: JoinHandle<Result<()>> = tokio::spawn(async move {
            let (mut sink, stream) = clienti.get_async_pubsub().await?.split();
            sink.subscribe("channel_1").await?;

            // 发布
            clienti.get_multiplexed_async_connection().await?.publish("channel_1", format!("t{} msg", i)).await?;

            // 订阅
            let stream = stream.timeout(Duration::from_millis(10));
            tokio::pin!(stream);
            while let Some(Ok(msg)) = stream.next().await {
                let payload: String = msg.get_payload()?;
                println!("t{}: {}", i, payload);
            }

            Ok(())
        });
        tasks.push(t);
    }

    let mut conn = client.get_multiplexed_async_connection().await?;

    conn.publish("channel_1", "hello").await?;
    conn.publish("channel_1", "world").await?;

    join_all(tasks).await;

    Ok(())
}

#[tokio::test]
async fn t_redis_resp3_pubsub() -> Result<()> {
    // let pool = Config::from_url("redis://localhost/?protocol=resp3").create_pool(None)?;
    // let conn = pool.get().await?;

    let client = redis::Client::open("redis://127.0.0.1/?protocol=resp3")?;
    let (tx, mut rx) = mpsc::unbounded_channel();
    let config = AsyncConnectionConfig::new().set_push_sender(tx);

    let mut con = client.get_multiplexed_async_connection_with_config(&config).await?;
    con.subscribe("channel_1").await?;

    println!("Received {:?}", rx.recv().await.unwrap());
    println!("Received {:?}", rx.recv().await.unwrap());
    let res = rx.recv().await.unwrap();
    for d in res.data {
        match d {
            Value::Nil => {}
            Value::Int(_) => {}
            Value::BulkString(_) => {}
            Value::Array(_) => {}
            Value::SimpleString(_) => {}
            Value::Okay => {}
            Value::Map(_) => {}
            Value::Attribute { .. } => {}
            Value::Set(_) => {}
            Value::Double(_) => {}
            Value::Boolean(_) => {}
            Value::VerbatimString { .. } => {}
            Value::BigNumber(_) => {}
            Value::Push { .. } => {}
            Value::ServerError(_) => {}
        }
    }

    // let mut conn = pool.get().await?;

    // AsyncConnectionConfig::new().set_push_sender()
    // conn.subscribe("channel_1").await?;
    // pool.

    Ok(())
}

#[tokio::test]
async fn t_redis_stream_pubsub() -> Result<()> {
    let pool = Config::from_url("redis://localhost/").create_pool(None)?;

    let mut tasks = vec![];
    for i in 0..2 {
        let mut conn = pool.get().await?;
        // 忽略错误 - 可能已存在
        let _: RedisResult<()> = conn.xgroup_create_mkstream("stream_1", format!("group_{}", i), "$").await;

        let options = StreamReadOptions::default()
            .group(format!("group_{}", i), format!("consumer_{}", i))
            .block(10);

        let t: JoinHandle<Result<()>> = tokio::spawn(async move {
            // 发布
            conn.xadd("stream_1", "*", &[("chat", format!("t{} msg", i))]).await?;

            // 订阅?
            loop {
                let reply: StreamReadReply = conn.xread_options(&["stream_1"], &[">"], &options).await?;

                if reply.keys.is_empty() {
                    break;
                }

                for key in &reply.keys {
                    let mut ids = vec![];
                    for id in &key.ids {
                        if let Value::BulkString(str) = &id.map["chat"] {
                            let name = String::from_utf8(str.to_vec())?;
                            println!("t{}: {}", i, name);
                            ids.push(&id.id);
                        }
                    }
                    conn.xack("stream_1", format!("group_{}", i), &ids).await?;
                }
            }

            Ok(())
        });
        tasks.push(t);
    }

    let mut conn = pool.get().await?;
    conn.xadd("stream_1", "*", &[("chat", "hello")]).await?;
    conn.xadd("stream_1", "*", &[("chat", "world")]).await?;

    join_all(tasks).await;

    // 清空消息
    pool.get().await?.del("stream_1").await?;

    Ok(())
}

#[tokio::test]
async fn t_multi_stream_forward() -> Result<()> {
    let mut int_stream = stream::iter(1..=10);
    let token = CancellationToken::new();
    let cloned_token = token.clone();


    let (tx, rx) = mpsc::channel::<()>(10);

    let rx_stream = ReceiverStream::new(rx);

    tokio::spawn(async move {
        token.cancel();
    });

    select! {
        int_item = int_stream.next() => {
            println!("int_item: {:?}", int_item);
        }
        _ = cloned_token.cancelled() => {
            println!("end");
        }
    }

    Ok(())
}
