use std::collections::HashMap;
use std::sync::RwLock;

use rocket::fairing::AdHoc;
use rocket::futures::channel::mpsc::UnboundedSender;
use rocket_ws::Message;

pub type ClientMap = RwLock<HashMap<u32, UnboundedSender<Message>>>;


pub fn stage() -> AdHoc {
    AdHoc::on_ignite("init chat", |rocket| async {
        let clients: ClientMap = RwLock::new(HashMap::new());

        rocket.manage(clients)
    })
}
