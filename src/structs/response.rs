use std::str::FromStr;

use super::request::Request;
use crate::enums::request_opts::{
    Headers,
    Method,
    HTTP
};

pub trait Httpify {
    fn to_http(&self) -> Vec<String>;
}

pub struct Response {
    pub status:  Status,
    pub content: Content
}

pub struct Status {
    pub http_version: HTTP,
    pub code:         u16,
    pub message:      String
}

pub struct Content {
    pub mime_type: String,
    pub data:      String
}

impl Default for Response {
    fn default() -> Self {
        return Self {
            status:  Status::new("HTTP/1.1".to_string(), 200, "OK".to_string()),
            content: Content::new(
                "text/html".to_string(),
                "<h1>test</h1>".to_string()
            )
        };
    }
}

impl Httpify for Response {
    fn to_http(&self) -> Vec<String> {
        let mut v = vec![];

        v.append(&mut self.status.to_http());
        v.append(&mut self.content.to_http());

        v
    }
}

impl Status {
    pub fn new(http_version: String, code: u16, message: String) -> Self {
        return Status {
            http_version: HTTP::from_str(&http_version).unwrap(),
            code,
            message
        };
    }
}

impl Httpify for Status {
    fn to_http(&self) -> Vec<String> {
        vec![format!(
            "{} {} {}",
            self.http_version, self.code, self.message
        )]
    }
}

impl Content {
    fn new(ty: String, data: String) -> Self { return Self { ty, data }; }
}

impl Httpify for Content {
    fn to_http(&self) -> Vec<String> {
        let mut content = vec![];

        content.push(format!("Content-Type: {}", self.ty));
        content.push(format!("Content-Length: {}", self.data.bytes().count()));
        content.push(format!(""));
        content.push(format!("{}", self.data));

        content
    }
}

impl From<Request> for Response {
    fn from(value: Request) -> Self {
        Response {
            status:  Status {
                http_version: value.client.http_version,
                code:         0,
                message:      "".to_string()
            },
            content: {}
        }
    }
}
