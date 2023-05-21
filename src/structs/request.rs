use crate::enums::{
    ip::IPType,
    request_opts::Method
};

pub struct Client {
    pub ip:         IPType,
    pub user_agent: Option<String>
}

pub struct Request {
    pub client: Client,

    pub method: Option<Method>,
    pub path:   Option<String>
}

impl Default for Request {
    fn default() -> Self {
        return Request {
            client: Client {
                ip:         IPType::NotAssigned,
                user_agent: None
            },
            method: None,
            path:   None
        };
    }
}
