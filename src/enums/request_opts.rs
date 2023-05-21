use std::str::FromStr;

pub enum Header {
    UserAgent(String),
    AcceptLanguage(Locale),
    AcceptEncoding(Vec<Encoding>),
    Accept(Vec<MimeType>)
}

pub type Headers = Vec<Header>;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub enum Method {
    UNSET,

    GET,
    POST
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
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

pub enum Encoding {
    Gzip,
    Compress,
    Deflate,
    Br,
    Identity,
    Asterix
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

pub enum Application {
    Javascript,
    Json,
    Mp4,
    OctetStream,
    Zip,
    Unknown(String)
}

pub enum Audio {
    XMidi,
    XWav
}

pub enum Image {
    Bmp,
    Gif,
    Jpeg,
    Tiff
}

pub enum Text {
    Html,
    Plain,
    Richtext,
    Css,
    Csv
}

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
                match mime_string.next().unwrap() {
                    "javascript" => {
                        MimeType::Application(Application::Javascript)
                    }
                    "json" => MimeType::Application(Application::Json),
                    "mp4" => MimeType::Application(Application::Mp4),
                    "octet-stream" => {
                        MimeType::Application(Application::OctetStream)
                    }
                    "zip" => MimeType::Application(Application::Zip),
                    x => {
                        MimeType::Application(Application::Unknown(
                            x.to_string()
                        ))
                    }
                }
            }

            _ => return Err(())
        };

        Ok(mime)
    }
}
