use std::future;
use rocket::futures::{SinkExt, StreamExt, TryStreamExt};
use rocket::futures::channel::mpsc::unbounded;
use rocket::State;
use rocket::tokio::try_join;
use rocket_ws::{
    Channel, Message, WebSocket,
    frame::{CloseCode, CloseFrame},
};
use openchat_bot::ChatBot;
use tokio::sync::broadcast::Sender;
use tokio_stream::wrappers::BroadcastStream;
use web_common::{
    core::resp::R,
};

rocket_base_path!("/chat");


/// 建立WebSocket连接, 全局聊天室
/// api文档不能WebSocket连接时发token, 所以这里用id来代替token
#[get("/connect/<id>")]
pub async fn connect(ws: WebSocket, id: u32, tx: &State<Sender<Message>>) -> Channel<'_> {
    ws.channel(move |stream| Box::pin(async move {
        let tx = &**tx;
        let (sender, receiver) = (tx.clone(), BroadcastStream::new(tx.subscribe()));

        let (write, read) = stream.split();

        sender.send(Message::Text(format!("{} 已上线", id))).unwrap();

        // 接收用户消息, 广播给其他用户
        let broadcast_task = read.try_for_each(|msg| {
            match msg {
                Message::Text(_) => {
                    sender.send(msg).unwrap();
                }
                Message::Close(close_msg) => {
                    println!("{:?}", close_msg);

                    sender.send(Message::Text(format!("{} 已下线", id))).unwrap();
                }
                _ => {}
            }

            future::ready(Ok(()))
        });

        // 接收其他用户消息, 转发给用户
        let forward_task = receiver
            .filter_map(|msg| future::ready(msg.ok()))
            .map(Ok)
            .forward(write);

        // todo check alive

        if let Err(err) = try_join!(broadcast_task, forward_task) {
            eprintln!("{}", err);

            info!("{} disconnect", id);
        }

        Ok(())
    }))
}

// #[utoipa::path(context_path = BASE)]
// #[delete("/<id>")]
// pub async fn kick(id: u32, clients: &State<ClientMap>) -> R {
//     clients.read().unwrap()[&id].unbounded_send(Message::Close(Some(CloseFrame { code: CloseCode::Normal, reason: "管理员踢出".into() }))).unwrap();
//     R::no_val_success()
// }

// #[utoipa::path(context_path = BASE)]
// #[get("/status")]
// pub async fn status(clients: &State<ClientMap>) -> R {
//     R::success(clients.read().unwrap().keys().collect::<Vec<_>>())
// }

// 聊天机器人
#[utoipa::path(context_path = BASE)]
#[get("/connect_bot/<id>")]
pub async fn connect_bot(ws: WebSocket, id: u32) -> Channel<'static> {
    ws.channel(move |mut stream| Box::pin(async move {
        let mut bot = ChatBot::from_default_args().await.unwrap();

        while let Some(Ok(Message::Text(msg))) = stream.next().await {
            let mut rx = bot.chat(msg);

            let cap = 32;
            let mut buf = Vec::with_capacity(cap);
            while rx.recv_many(&mut buf, cap).await > 0 {
                stream.send(Message::Text(buf.join(""))).await?;
                buf.clear();
            }
        }

        Ok(())
    }))
}
