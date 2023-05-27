use {
    super::request::RequestParser,
    crate::request::headers::{
        http_version::HTTPVersion,
        locale::Locale,
        method::Method,
        Header
    },
    std::{
        assert_matches::assert_matches,
        fs::File,
        io::Read
    }
};

const REQ_PATH: &'static str =
    "/Users/vii/plane_rs/crates/plane_rs_types/test_data/requests/req2.txt";

fn make_stream() -> anyhow::Result<RequestParser> {
    let file = File::open(REQ_PATH)?;

    Ok(RequestParser::new(file))
}

#[test]
fn parse() -> anyhow::Result<()> {
    let mut s = make_stream()?;
    s.parse()?;

    assert_eq!(s.req.route, "/".to_string());
    assert_matches!(s.req.client.http_version, HTTPVersion::V1_1);
    assert_matches!(s.req.method, Method::GET);
    assert!(
        s.req
            .client
            .headers
            .contains(&Header::AcceptLanguage(Locale::new("en", "GB")))
    );
    Ok(())
}
