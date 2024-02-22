use std::future;

use rocket::futures::{StreamExt, TryStreamExt};
use rocket::futures::channel::mpsc;
use rocket::State;
use rocket::tokio::try_join;
use rocket_ws::{Channel, Message, WebSocket};

use crate::ClientMap;

#[utoipa::path]
#[get("/connect/<id>")]
pub fn connect(ws: WebSocket, id: u32, clients: &State<ClientMap>) -> Channel<'_> {
    ws.channel(move |stream| Box::pin(async move {
        let (sender, receiver) = mpsc::unbounded();

        let (write, read) = stream.split();

        clients.lock().unwrap().insert(id, sender);

        clients.lock().unwrap().iter().for_each(|(_, sender)| sender.unbounded_send(Message::Text(format!("{} 已上线", id))).unwrap());

        let broadcast = read.try_for_each(|msg| {
            match msg {
                Message::Text(_) => {
                    clients.lock().unwrap().iter()
                        .filter(|(&mid, sender)| id != mid && !sender.is_closed())
                        .for_each(|(_, sender)| sender.unbounded_send(msg.clone()).unwrap());
                }
                Message::Close(_) => {
                    clients.lock().unwrap().remove(&id);

                    clients.lock().unwrap().iter()
                        .for_each(|(_, sender)| sender.unbounded_send(Message::Text(format!("{} 已下线", id))).unwrap());
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
