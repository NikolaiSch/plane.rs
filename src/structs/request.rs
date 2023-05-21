use crate::enums::{
    ip::IPType,
    request_opts::{
        Headers,
        Method
    }
};

pub struct Client {
    pub ip:      IPType,
    pub headers: Headers
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
                ip:      IPType::NotAssigned,
                headers: vec![]
            },
            method: Method::UNSET,
            route:  "/".to_string()
        };
    }
}
