use std::collections::HashMap;
use std::sync::Mutex;

use rocket::fairing::AdHoc;
use rocket::futures::channel::mpsc::UnboundedSender;
use rocket_ws::Message;

pub type ClientMap = Mutex<HashMap<u32, UnboundedSender<Message>>>;

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("init chat", |rocket| async {
        let clients: ClientMap = Mutex::new(HashMap::new());

        rocket.manage(clients)
    })
}
