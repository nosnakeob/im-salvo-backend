use std::collections::HashMap;
use std::sync::Mutex;

use rocket::futures::channel::mpsc::UnboundedSender;
use rocket_ws::Message;

use crate::framework::rocket::Server;

pub type ClientMap = Mutex<HashMap<u32, UnboundedSender<Message>>>;

impl Server {
    pub fn init_chat(mut self) -> Self {
        let clients: ClientMap = Mutex::new(HashMap::new());

        self.0 = self.0.manage(clients);

        self
    }
}