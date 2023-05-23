use crate::enums::{
    ip::IPType,
    request_opts::{
        Headers,
        Method,
        HTTP
    }
};

pub struct Client {
    pub ip:       IPType,
    pub headers:  Headers,
    http_version: HTTP
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
                ip:           IPType::Unassigned,
                http_version: HTTP::Unassigned,
                headers:      vec![]
            },
            method: Method::Unassigned,
            route:  "/".to_string()
        };
    }
}
