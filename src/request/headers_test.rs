use {
    super::{
        headers::Header,
        *
    },
    std::{
        assert_matches::assert_matches,
        fs::File,
        io::Read,
        str::FromStr
    }
};

fn parse_file(path: &str) -> Vec<anyhow::Result<Header>> {
    let mut s = String::new();
    File::open(path).unwrap().read_to_string(&mut s).unwrap();

    return s.lines().map(Header::from_str).collect();
}

#[test]
fn parse_user_agents() {
    let headers = parse_file(
        "/Users/vii/plane_rs/crates/plane_rs_types/test_data/headers/\
         user-agent.txt"
    );

    for header in headers {
        assert_matches!(header.unwrap(), Header::UserAgent(_));
    }
}

#[test]
fn parse_locales() {
    let headers = parse_file(
        "/Users/vii/plane_rs/crates/plane_rs_types/test_data/headers/locale.\
         txt"
    );

    for header in headers {
        assert_matches!(header.unwrap(), Header::AcceptLanguage(_));
    }
}

#[test]
fn parse_encodings() {
    let headers = parse_file(
        "/Users/vii/plane_rs/crates/plane_rs_types/test_data/headers/\
         encodings.txt"
    );

    for header in headers {
        assert_matches!(header.unwrap(), Header::AcceptEncoding(_));
    }
}

#[test]
fn parse_mime_types() {
    let headers = parse_file(
        "/Users/vii/plane_rs/crates/plane_rs_types/test_data/headers/mime.txt"
    );

    for header in headers {
        assert_matches!(header.unwrap(), Header::Accept(_));
    }
}

#[test]
fn parse_mixed() {
    let headers = parse_file(
        "/Users/vii/plane_rs/crates/plane_rs_types/test_data/headers/mixed.txt"
    );

    for header in headers {
        if let Ok(x) = header {
            // dbg!(x);
        }
    }
}
