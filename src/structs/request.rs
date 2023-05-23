use crate::enums::{
    ip::IPType,
    request_opts::{
        HTTPStatus,
        Headers,
        Method
    }
};

#[derive(Debug)]
pub struct Client {
    pub ip:           IPType,
    pub headers:      Headers,
    pub http_version: HTTPStatus
}

#[derive(Debug)]
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
                http_version: HTTPStatus::Unassigned,
                headers:      vec![]
            },
            method: Method::Unassigned,
            route:  "/".to_string()
        };
    }
}
