use rocket::http::{Header, Status};
use crate::domain::user::User;
use web_common::core::resp::Resp;
use crate::test::get_client;

#[test]
fn register() {
    let client = get_client();
    let resp = client.post("/auth/register")
        .json(&User::default())
        .dispatch();

    let status = resp.status();

    if status != Status::Ok {
        println!("{:#?}", resp.into_json::<Resp>().unwrap());
    }

    // 首次注册
    // assert_eq!(resp.status(), Status::Ok);
    assert_eq!(status, Status::BadRequest);
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
