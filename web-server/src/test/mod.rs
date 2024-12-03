use qrcode::render::unicode;
use qrcode::QrCode;
use rocket::http::Status;
use rocket::local::blocking::Client;
use web_common::core::resp::Resp;
use crate::build_rocket;
use crate::domain::chat::ChatMessage;

mod auth;
mod demo;
mod lock;
mod chat;

fn get_client() -> Client {
    Client::tracked(build_rocket()).unwrap()
}

#[test]
fn index() {
    let client = get_client();
    let resp = client.get("/").dispatch();

    let status = resp.status();

    if status != Status::Ok {
        println!("{:#?}", resp.into_json::<Resp>().unwrap());
    }

    assert_eq!(status, Status::Ok);
}

#[test]
fn qrcode() {
    let a = 123;
    let code = QrCode::new(format!("http://localhost:8000/lock/{}", a)).unwrap();
    let image = code.render::<unicode::Dense1x2>()
        .dark_color(unicode::Dense1x2::Light)
        .light_color(unicode::Dense1x2::Dark)
        .build();
    println!("{}", image);
}

#[test]
fn serde() {
    let msg = r#"{
        "username": "snake1",
        "content": "hello"
    }"#;
    let msg: ChatMessage = serde_json::from_str(msg).unwrap();
    println!("{:?}", msg);
}