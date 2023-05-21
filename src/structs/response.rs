use crate::enums::request_opts::{
    Headers,
    Method
};

pub trait Httpify {
    fn to_http_string(&self) -> String;
}

pub struct Response {
    pub status:  Status,
    pub content: Content
}

impl Default for Response {
    fn default() -> Self {
        return Self {
            status:  Status::new("HTTP/1.1".to_string(), 200, "OK".to_string()),
            content: Content::new("text/plain".to_string(), "test".to_string())
        };
    }
}

impl Httpify for Response {
    fn to_http_string(&self) -> String {
        let mut v = vec![];

        v.push(self.status.to_http_string());
        v.push(self.content.to_http_string());

        return v.join("\\n");
    }
}

pub struct Status {
    http_version: String,
    code:         u16,
    message:      String
}

impl Status {
    pub fn new(
        http_version: String,
        code: u16,
        message: String
    ) -> Self {
        return Status {
            http_version,
            code,
            message
        };
    }
}

impl Httpify for Status {
    fn to_http_string(&self) -> String {
        format!("{} {} {}", self.http_version, self.code, self.message)
    }
}

pub struct Content {
    pub ty:   String,
    pub data: String
}

impl Content {
    fn new(
        ty: String,
        data: String
    ) -> Self {
        return Self { ty, data };
    }
}

impl Httpify for Content {
    fn to_http_string(&self) -> String {
        let mut content = vec![];

        content.push(format!("Content-Type: {}", self.ty));
        content.push(format!("Content-Length: {}", self.data.bytes().count()));
        content.push(format!("\\n{}", self.data));

        content.join("\\n")
    }
}
