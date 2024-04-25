use rocket::http::Status;
use rocket::local::blocking::Client;
use crate::framework::rocket::resp::Resp;

use crate::rocket;

mod auth;

fn get_client() -> Client {
    Client::tracked(rocket()).unwrap()
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

