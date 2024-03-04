use md5::compute;

pub fn encode<S: Into<String>>(raw_password: S) -> String {
    let digest = compute(raw_password.into());
    format!("{:x}", digest)
}

pub fn verify<S: Into<String>, U: Into<String>>(password: S, raw_password: U) -> bool {
    let hashed = encode(raw_password);
    password.into() == hashed
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

    assert!(verify(encode(password), raw_password));
}