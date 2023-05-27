use {
    crate::request::errors::HeaderErrors,
    std::str::FromStr,
    strum::EnumString,
    Application::*,
    Audio::*,
    Image::*,
    Text::*,
    Video::*
};

#[derive(Debug, EnumString, PartialEq, Eq)]
#[strum(serialize_all = "snake_case")]
#[strum(ascii_case_insensitive)]
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

impl MimeType {
    pub fn get_subtype(&self, s: &str) -> anyhow::Result<MimeType> {
        match self {
            MimeType::Application(_) => {
                Ok(MimeType::Application(Application::from_str(s)?))
            }
            MimeType::Audio(_) => Ok(MimeType::Audio(Audio::from_str(s)?)),
            MimeType::Image(_) => Ok(MimeType::Image(Image::from_str(s)?)),
            MimeType::Text(_) => Ok(MimeType::Text(Text::from_str(s)?)),
            MimeType::Video(_) => Ok(MimeType::Video(Video::from_str(s)?)),
            _ => {
                Err(HeaderErrors::MimeTypeError(
                    "Could not parse Mimetype".to_string()
                )
                .into())
            }
        }
    }

    pub fn from_header(s: &str) -> anyhow::Result<MimeType> {
        let mut s_array: [&str; 2] = Default::default();

        s.split("/").enumerate().for_each(|(i, x)| s_array[i] = x);
        let mime = MimeType::from_str(s_array[0]);

        if let Err(_) = mime {
            return Ok(MimeType::Unknown(s.to_string()));
        } else {
            return Ok(mime
                .unwrap()
                .get_subtype(s_array[1])
                .unwrap_or(MimeType::Unknown(s.to_string())));
        }
    }
}

#[derive(Debug, EnumString, PartialEq, Eq, Default)]
#[strum(serialize_all = "snake_case")]
#[strum(ascii_case_insensitive)]
pub enum Application {
    #[default]
    Javascript,
    Json,
    Mp4,
    OctetStream,
    Zip
}

#[derive(Debug, EnumString, PartialEq, Eq, Default)]
#[strum(serialize_all = "snake_case")]
#[strum(ascii_case_insensitive)]
pub enum Audio {
    #[default]
    XMidi,
    XWav
}

#[derive(Debug, EnumString, PartialEq, Eq, Default)]
#[strum(serialize_all = "snake_case")]
#[strum(ascii_case_insensitive)]
pub enum Image {
    #[default]
    Bmp,
    Gif,
    Jpeg,
    Tiff
}

#[derive(Debug, EnumString, PartialEq, Eq, Default)]
#[strum(serialize_all = "snake_case")]
#[strum(ascii_case_insensitive)]
pub enum Text {
    #[default]
    Html,
    Plain,
    Richtext,
    Css,
    Csv
}

#[derive(Debug, EnumString, PartialEq, Eq, Default)]
#[strum(serialize_all = "snake_case")]
#[strum(ascii_case_insensitive)]
pub enum Video {
    #[default]
    Mpeg,
    Quicktime
}
