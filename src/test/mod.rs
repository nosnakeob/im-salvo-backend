use rocket::http::{Header, Status};
use rocket::local::blocking::Client;

use crate::domain::user::User;
use crate::framework::rocket::resp::Resp;
use crate::rocket;

fn get_client() -> Client {
    Client::tracked(rocket()).unwrap()
}

#[test]
fn index() {
    let client = get_client();
    let resp = client.get("/").dispatch();

    assert_eq!(resp.status(), Status::Ok);
}

#[test]
fn register() {
    let client = get_client();
    let resp = client.post("/auth/register")
        .json(&User::default())
        .dispatch();

    // 首次注册
    // assert_eq!(resp.status(), Status::Ok);
    assert_eq!(resp.status(), Status::BadRequest);

    println!("{:#?}", resp.into_json::<Resp>());
}


#[test]
fn login() {
    // 已有对应用户
    let client = get_client();
    let mut resp = client.post("/auth/login")
        .json(&User::default())
        .dispatch();

    // assert_eq!(resp.status(), Status::Ok);
    let mut rj: Resp = resp.into_json().unwrap();
    println!("{:#?}", rj);

    let token = rj.data.unwrap().as_object().unwrap()
        .get("token").unwrap().as_str().unwrap().to_string();

    resp = client.get("/auth/check")
        .header(Header::new("Authorization", format!("Bearer {}", token)))
        .dispatch();

    assert_eq!(resp.status(), Status::Ok);

    rj = resp.into_json().unwrap();
    println!("{:#?}", rj);
}
