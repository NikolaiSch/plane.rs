use std::{
    fmt::{
        Display,
        Pointer
    },
    str::FromStr
};

#[derive(Debug)]
pub enum Header {
    UserAgent(String),
    AcceptLanguage(Locale),
    AcceptEncoding(Vec<Encoding>),
    Accept(Vec<MimeType>)
}

pub type Headers = Vec<Header>;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug)]
pub enum Method {
    Unassigned,

    GET,
    POST
}

#[derive(Debug)]
pub enum HTTPStatus {
    Unassigned,

    V1_0,
    V1_1,
    V2,
    V3
}

#[derive(Debug)]
pub enum Encoding {
    Unassigned,

    Gzip,
    Compress,
    Deflate,
    Br,
    Identity,
    Asterix
}

#[derive(Debug)]
pub enum MimeType {
    Application(Application),
    Audio(Audio),
    Font,
    Image(Image),
    Model,
    Text(Text),
    Video(Video),
    Multipart,
    Unknown(String)
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Locale {
    language: String,
    country:  String
}

impl FromStr for Locale {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut sp_s = s.split("-");

        return Ok(Self {
            language: sp_s.next().unwrap().to_string(),
            country:  sp_s.next().unwrap().to_string()
        });
    }
}

impl FromStr for Encoding {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "gzip" => Ok(Self::Gzip),
            "compress" => Ok(Self::Compress),
            "deflate" => Ok(Self::Deflate),
            "br" => Ok(Self::Br),
            "identity" => Ok(Self::Identity),
            "*" => Ok(Self::Asterix),
            _ => Err(())
        }
    }
}

#[derive(Debug)]
pub enum Application {
    Javascript,
    Json,
    Mp4,
    OctetStream,
    Zip,
    Unknown(String)
}
#[derive(Debug)]
pub enum Audio {
    XMidi,
    XWav
}
#[derive(Debug)]
pub enum Image {
    Bmp,
    Gif,
    Jpeg,
    Tiff
}
#[derive(Debug)]
pub enum Text {
    Html,
    Plain,
    Richtext,
    Css,
    Csv
}
#[derive(Debug)]
pub enum Video {
    Mpeg,
    Quicktime
}

impl FromStr for MimeType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut mime_string = s.split("/");
        let mime = match mime_string.next().unwrap() {
            "application" => {
                MimeType::Application(match mime_string.next().unwrap() {
                    "javascript" => Application::Javascript,
                    "json" => Application::Json,
                    "mp4" => Application::Mp4,
                    "octet-stream" => Application::OctetStream,
                    "zip" => Application::Zip,
                    x => Application::Unknown(x.to_string())
                })
            }

            _ => return Err(())
        };

        Ok(mime)
    }
}

impl FromStr for Method {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(Method::GET),
            "POST" => Ok(Method::POST),
            _ => Err(())
        }
    }
}

impl FromStr for HTTPStatus {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "HTTP/1.0" => Ok(HTTPStatus::V1_0),
            "HTTP/1.1" => Ok(HTTPStatus::V1_1),
            "HTTP/2" => Ok(HTTPStatus::V2),
            "HTTP/3" => Ok(HTTPStatus::V3),
            _ => Err(())
        }
    }
}

impl Display for HTTPStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            HTTPStatus::V1_0 => "HTTP/1.0",
            HTTPStatus::V1_1 => "HTTP/1.1",
            HTTPStatus::V2 => "HTTP/2",
            HTTPStatus::V3 => "HTTP/3",

            HTTPStatus::Unassigned => return Err(std::fmt::Error)
        };

        f.write_str(s)
    }
}
