use super::headers::{
    http_version::HTTPVersion,
    method::Method,
    Header
};

pub struct Client {
    pub headers:      Vec<Header>,
    pub http_version: HTTPVersion
}

pub struct Request {
    pub client: Client,

    pub method: Method,
    pub route:  String
}

impl Default for Request {
    fn default() -> Self {
        return Request {
            client: Client {
                http_version: HTTPVersion::Unassigned,
                headers:      vec![]
            },
            method: Method::Unassigned,
            route:  "/".to_string()
        };
    }
}
