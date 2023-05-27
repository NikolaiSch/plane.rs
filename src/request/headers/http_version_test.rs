use std::str::FromStr;

use super::{
    super::errors::HeaderErrors,
    http_version::*
};

fn check_version(s: &str, correct_version: HTTPVersion) -> () {
    let equal = HTTPVersion::from_str(s)
        .is_ok_and(|version| version.eq(&correct_version));

    assert!(
        equal,
        "Version had error with variant: {:?}, input: {}",
        correct_version, s
    );

    assert_eq!(&*correct_version.to_string(), s);
}

#[test]
fn v1_0() { check_version("HTTP/1.0", HTTPVersion::V1_0); }

#[test]
fn v1_1() { check_version("HTTP/1.1", HTTPVersion::V1_1); }

#[test]
fn v2() { check_version("HTTP/2", HTTPVersion::V2); }

#[test]
fn v3() { check_version("HTTP/3", HTTPVersion::V3); }

#[test]
fn invalid() {
    let e = HTTPVersion::from_str("panic");
    assert!(e.is_err());

    assert!(e.is_err_and(|e| e.is::<HeaderErrors>()));
}
