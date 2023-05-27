use {
    super::{
        super::errors::HeaderErrors,
        mime_types::*
    },
    std::str::FromStr
};

fn check_mime(s: &str, mime: MimeType) -> () {
    let m = MimeType::from_header(s);
    assert!(m.is_ok());

    assert_eq!(
        m.unwrap(),
        mime,
        "Mime had error with variant: {:?}, input: {}",
        mime,
        s
    );
}

#[test]
fn text_plain() {
    check_mime("text/plain", MimeType::Text(Text::Plain));
}

#[test]
fn application_javascript() {
    check_mime(
        "application/javascript",
        MimeType::Application(Application::Javascript)
    );
}

#[test]
fn image_gif() {
    check_mime("image/gif", MimeType::Image(Image::Gif));
}

#[test]
fn video_mpeg() {
    check_mime("video/mpeg", MimeType::Video(Video::Mpeg));
}

#[test]
fn invalid() {
    let e = MimeType::from_str("panic");
    assert!(e.is_err());
}
