use {
    super::response_codes::StatusCodes,
    crate::request::{
        headers::{
            http_version::HTTPVersion,
            mime_types::*
        },
        request::Request
    },
    std::{
        default,
        str::FromStr
    }
};

pub trait Httpify {
    fn to_http(&self) -> Vec<String>;
}

pub struct Response {
    pub status:  Status,
    pub content: Content
}

pub struct Status {
    pub http_version: HTTPVersion,
    pub code:         u16,
    pub message:      String
}

pub struct Content {
    pub mime_type: MimeType,
    pub data:      String
}

impl Default for Content {
    fn default() -> Self {
        Self {
            data:      "<h1>test</h1>".to_string(),
            mime_type: MimeType::Text(Text::Html)
        }
    }
}

impl Default for Response {
    fn default() -> Self {
        return Self {
            ..Default::default()
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
            http_version: HTTPVersion::from_str(&http_version).unwrap(),
            code,
            message
        };
    }
}

impl Default for Status {
    fn default() -> Self {
        Status::new("HTTP/1.1".to_string(), 200, "OK".to_string())
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
    pub fn new(mime_type: &str, data: &str) -> Self {
        return Self {
            mime_type: mime_type.to_string(),
            data:      data.to_string()
        };
    }
}

impl Httpify for Content {
    fn to_http(&self) -> Vec<String> {
        let mut content = vec![];

        content.push(format!("Content-Type: {}", self.mime_type));
        content.push(format!("Content-Length: {}", self.data.bytes().count()));
        content.push(format!(""));
        content.push(format!("{}", self.data));

        content
    }
}

impl From<Request> for Response {
    fn from(value: Request) -> Self {
        Response {
            status:  StatusCodes::OK.get(),
            content: Content {
                mime_type: "text/html".to_string(),
                data:      "hello from plane".to_string()
            }
        }
    }
}
