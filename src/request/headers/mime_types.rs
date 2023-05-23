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
