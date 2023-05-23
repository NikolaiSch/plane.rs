pub mod encoding;
pub mod http_version;
pub mod locale;
pub mod method;
pub mod mime_types;

pub enum Header {
    UserAgent(String),
    AcceptLanguage(Locale),
    AcceptEncoding(Vec<Encoding>),
    Accept(Vec<MimeType>)
}
