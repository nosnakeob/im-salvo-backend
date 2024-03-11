use std::future;

use rocket::futures::{StreamExt, TryStreamExt};
use rocket::futures::channel::mpsc;
use rocket::State;
use rocket::tokio::try_join;
use rocket_ws::{Channel, Message, WebSocket};
use rocket_ws::frame::{CloseCode, CloseFrame};
use serde_json::json;

use crate::framework::rocket::resp::R;
use crate::framework::websocket::ClientMap;

rocket_base_path!("/chat");

// #[utoipa::path]
#[get("/connect/<id>")]
pub async fn connect(ws: WebSocket, id: u32, clients: &State<ClientMap>) -> Channel<'_> {
    ws.channel(move |stream| Box::pin(async move {
        let (sender, receiver) = mpsc::unbounded();

        let (write, read) = stream.split();

        clients.lock().unwrap().insert(id, sender);

        clients.lock().unwrap().iter().for_each(|(_, sender)| sender.unbounded_send(Message::Text(format!("{} 已上线", id))).unwrap());

        let broadcast = read.try_for_each(|msg| {
            match msg {
                Message::Text(_) => {
                    clients.lock().unwrap().iter()
                        .filter(|(&mid, _)| id != mid)
                        .for_each(|(_, sender)| sender.unbounded_send(msg.clone()).unwrap());
                }
                Message::Close(close_msg) => {
                    println!("{:?}", close_msg);

                    let mut guard = clients.lock().unwrap();

                    guard.remove(&id);

                    guard.iter().for_each(|(_, sender)|
                        sender.unbounded_send(Message::Text(format!("{} 已下线", id))).unwrap()
                    );
                }
                _ => {}
            }

            future::ready(Ok(()))
        });

        let recv = receiver.map(Ok).forward(write);

        if let Err(err) = try_join!(broadcast, recv) {
            eprintln!("{}", err);

            clients.lock().unwrap().remove(&id);

            println!("{} disconnect", id);
        }

        Ok(())
    }))
}

#[utoipa::path(context_path = "/chat")]
#[delete("/<id>")]
pub async fn kick(id: u32, clients: &State<ClientMap>) -> R {
    clients.lock().unwrap()[&id].unbounded_send(Message::Close(Some(CloseFrame { code: CloseCode::Normal, reason: "管理员踢出".into() }))).unwrap();
    R::Success(None::<u8>.into())
}

#[utoipa::path(context_path = "/chat")]
#[get("/status")]
pub async fn status(clients: &State<ClientMap>) -> R {
    R::Success(Some(json!(clients.lock().unwrap().keys().collect::<Vec<_>>())).into())
}