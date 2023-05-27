use {
    super::{
        super::errors::HeaderErrors,
        method::*
    },
    std::str::FromStr
};

fn check_version(s: &str, method: Method) -> () {
    let equal = Method::from_str(s).is_ok_and(|m| m.eq(&method));

    assert!(
        equal,
        "Version had error with variant: {:?}, input: {}",
        method, s
    );
}

#[test]
fn get() {
    check_version("GET", Method::GET);
}

#[test]
fn post() {
    check_version("POST", Method::POST);
}

#[test]
fn put() {
    check_version("PUT", Method::PUT);
}

#[test]
fn patch() {
    check_version("PATCH", Method::PATCH);
}

#[test]
fn delete() {
    check_version("DELETE", Method::DELETE);
}

#[test]
fn options() {
    check_version("OPTIONS", Method::OPTIONS);
}

#[test]
fn head() {
    check_version("HEAD", Method::HEAD);
}

#[test]
fn invalid() {
    let e = Method::from_str("panic");
    assert!(e.is_err());

    assert!(e.is_err_and(|e| e.is::<HeaderErrors>()));
}
