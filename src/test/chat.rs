use anyhow::Result;
use deadpool_redis::Config;
use redis::streams::{StreamReadOptions, StreamReadReply};
use redis::{AsyncCommands, Commands, PubSubCommands, RedisResult, Value};
use rocket::futures::future::join_all;
use rocket::futures::SinkExt;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::broadcast;
use tokio::task::JoinHandle;
use tokio::time::timeout;
use tokio_stream::StreamExt;

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
            let (mut sink, mut stream) = clienti.get_async_pubsub().await?.split();
            sink.subscribe("channel_1").await?;

            // 发布
            clienti.get_multiplexed_async_connection().await?.publish("channel_1", format!("t{} msg", i)).await?;

            // 订阅
            let mut stream = stream.timeout(Duration::from_millis(10));
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
                let mut reply: StreamReadReply = conn.xread_options(&["stream_1"], &[">"], &options).await?;

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