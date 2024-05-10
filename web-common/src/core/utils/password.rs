use md5::compute;

pub fn encode(raw_password: &str) -> String {
    let digest = compute(raw_password);
    format!("{:x}", digest)
}

pub fn verify(password: &str, raw_password: &str) -> bool {
    let hashed = encode(raw_password);
    password == hashed
}

#[test]
fn t() {
    println!("{}", "123" == "123");
    println!("{}", String::from("123") == "123");
}

#[test]
fn test_encode() {
    let s = encode("123456");
    println!("{}", s);
    assert_eq!(
        encode("123456"),
        encode("123456")
    );
}

#[test]
fn test_verify() {
    let password = "12345";
    let raw_password = "12345";

    assert!(!verify(password, raw_password));

    assert!(verify(&encode(password), raw_password));
}