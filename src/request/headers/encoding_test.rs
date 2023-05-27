use std::str::FromStr;

use super::{
    super::errors::HeaderErrors,
    encoding::*
};

fn check_encoding(s: &str, enc: Encoding) -> () {
    let equal = Encoding::from_str(s).is_ok_and(|e| e.eq(&enc));

    assert!(
        equal,
        "Encoding had error with variant: {:?}, input: {}",
        enc, s
    )
}

#[test]
fn gzip() { check_encoding("gzip", Encoding::Gzip); }

#[test]
fn compress() { check_encoding("compress", Encoding::Compress); }

#[test]
fn deflate() { check_encoding("deflate", Encoding::Deflate); }

#[test]
fn brotli() { check_encoding("br", Encoding::Br); }

#[test]
fn identity() { check_encoding("identity", Encoding::Identity); }

#[test]
fn asterix() { check_encoding("*", Encoding::Asterix); }

#[test]
fn invalid() {
    let e = Encoding::from_str("panic");
    assert!(e.is_err());

    assert!(e.is_err_and(|e| e.is::<HeaderErrors>()));
}
