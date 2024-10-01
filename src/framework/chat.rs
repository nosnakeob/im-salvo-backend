use rocket::fairing::AdHoc;
use rocket_ws::Message;
use tokio::sync::broadcast;


pub fn stage() -> AdHoc {
    AdHoc::on_ignite("init chat", |rocket| async {
        let (tx,_) = broadcast::channel::<Message>(1024);

        rocket.manage(tx)
    })
}
