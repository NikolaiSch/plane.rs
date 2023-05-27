use std::str::FromStr;

use super::{
    super::errors::HeaderErrors,
    locale::*
};

fn check_locale(s: &str, locale: Locale) -> () {
    let equal = Locale::from_str(s).is_ok_and(|l| l.eq(&locale));

    assert!(
        equal,
        "Locale had error with variant: {:?}, input: {}",
        locale, s
    );
}

#[test]
fn unitedkingdom() { check_locale("en-GB", Locale::new("en", "GB")); }

#[test]
fn china() { check_locale("zh-CN", Locale::new("zh", "CN")); }

#[test]
fn invalid() {
    let e = Locale::from_str("panic");
    assert!(e.is_err());

    assert!(e.is_err_and(|e| { e.is::<HeaderErrors>() }));
}

fn error_message(input: &str, expected: &str) {
    let e = Locale::from_str(input);
    assert!(e.as_ref().is_err());
    assert!(e.as_ref().is_err_and(|e| { e.is::<HeaderErrors>() }));

    assert_eq!(e.err().unwrap().to_string(), expected.to_string());
}

#[test]
fn err_too_long() {
    error_message(
        "1111111",
        "Invalid Locale: Malformed Input: Expected exactly 5 characters"
    )
}

#[test]
fn err_not_alphabetic() {
    error_message(
        "%%-%%",
        "Invalid Locale: Malformed Input: Expected only alphabetic characters"
    )
}

#[test]
fn err_else() {
    error_message(
        "aa)aa",
        "Invalid Locale: Malformed Input: Expected format 'xx-xx'"
    )
}
