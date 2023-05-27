use mime::Mime;

pub trait MimeParser {
    type MimeType;
    fn mime_type() -> Mime;
}
pub trait FromReq: MimeParser {
    fn from_req_body(body: String) -> anyhow::Result<Self>
    where
        Self: Sized;
}

pub trait ToRes: MimeParser {
    fn to_req_body(body: Self::MimeType) -> anyhow::Result<Self>
    where
        Self: Sized;
}
