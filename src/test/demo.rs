use rocket::http::Status;
use crate::framework::rocket::resp::Resp;
use crate::test::get_client;

#[test]
fn transaction() {
    let client = get_client();
    let resp = client.get("/demo/transaction").dispatch();

    let status = resp.status();

    if status != Status::Ok {
        println!("{:#?}", resp.into_json::<Resp>().unwrap());
    }

    assert_eq!(status, Status::Ok);
}

#[test]
fn config() {
    let client = get_client();
    let resp = client.get("/demo/config").dispatch();

    let status = resp.status();

    if status != Status::Ok {
        println!("{:#?}", resp.into_json::<Resp>().unwrap());
    }

    assert_eq!(status, Status::Ok);
}