use std::str::FromStr;

use super::{
    super::errors::HeaderErrors,
    method::*
};

fn check_version(s: &str, method: MimeType) -> () {
    let equal = MimeType::from_str(s).is_ok_and(|m| m.eq(&method));

    assert!(
        equal,
        "Version had error with variant: {:?}, input: {}",
        method, s
    );
}

#[test]
fn get() { check_version("GET", MimeType::GET); }

#[test]
fn post() { check_version("POST", MimeType::POST); }

#[test]
fn put() { check_version("PUT", MimeType::PUT); }

#[test]
fn patch() { check_version("PATCH", MimeType::PATCH); }

#[test]
fn delete() { check_version("DELETE", MimeType::DELETE); }

#[test]
fn options() { check_version("OPTIONS", MimeType::OPTIONS); }

#[test]
fn head() { check_version("HEAD", MimeType::HEAD); }

#[test]
fn invalid() {
    let e = MimeType::from_str("panic");
    assert!(e.is_err());

    assert!(e.is_err_and(|e| e.is::<HeaderErrors>()));
}
