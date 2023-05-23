use crate::enums::request_opts::{
    Headers,
    Method
};

pub trait Httpify {
    fn to_http(&self) -> Vec<String>;
}

pub struct Response {
    pub status:  Status,
    pub content: Content
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
    fn to_http(&self) -> Vec<String> {
        vec![format!(
            "{} {} {}",
            self.http_version, self.code, self.message
        )]
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
    fn to_http(&self) -> Vec<String> {
        let mut content = vec![];

        content.push(format!("Content-Type: {}", self.ty));
        content.push(format!("Content-Length: {}", self.data.bytes().count()));
        content.push(format!(""));
        content.push(format!("{}", self.data));

        content
    }
}
