use std::future;

use rocket::futures::{StreamExt, TryStreamExt};
use rocket::futures::channel::mpsc;
use rocket::State;
use rocket::tokio::try_join;
use rocket_ws::{Channel, Message, WebSocket};
use rocket_ws::frame::{CloseCode, CloseFrame};

use crate::framework::rocket::resp::R;
use crate::framework::websocket::ClientMap;

rocket_base_path!("/chat");

#[get("/connect/<id>")]
pub async fn connect(ws: WebSocket, id: u32, clients: &State<ClientMap>) -> Channel<'_> {
    ws.channel(move |stream| Box::pin(async move {
        let (sender, receiver) = mpsc::unbounded();

        let (write, read) = stream.split();

        clients.write().unwrap().insert(id, sender);

        clients.read().unwrap().iter().for_each(|(_, sender)| sender.unbounded_send(Message::Text(format!("{} 已上线", id))).unwrap());

        let broadcast = read.try_for_each(|msg| {
            match msg {
                Message::Text(_) => {
                    clients.read().unwrap().iter()
                        .filter(|(&mid, _)| id != mid)
                        .for_each(|(_, sender)| sender.unbounded_send(msg.clone()).unwrap());
                }
                Message::Close(close_msg) => {
                    println!("{:?}", close_msg);

                    let mut guard = clients.write().unwrap();

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

            clients.write().unwrap().remove(&id);

            println!("{} disconnect", id);
        }

        Ok(())
    }))
}

#[utoipa::path(context_path = BASE)]
#[delete("/<id>")]
pub async fn kick(id: u32, clients: &State<ClientMap>) -> R {
    clients.read().unwrap()[&id].unbounded_send(Message::Close(Some(CloseFrame { code: CloseCode::Normal, reason: "管理员踢出".into() }))).unwrap();
    R::no_val_success()
}

#[utoipa::path(context_path = BASE)]
#[get("/status")]
pub async fn status(clients: &State<ClientMap>) -> R {
    R::success(clients.read().unwrap().keys().collect::<Vec<_>>())
}