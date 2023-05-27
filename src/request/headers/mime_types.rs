use std::str::FromStr;

use strum::EnumString;

use crate::request::errors::HeaderErrors;

#[derive(Debug)]
pub struct MimeType {
    pub super_type: MimeSuperType,
    pub sub_type:   MimeSubType
}

impl FromStr for MimeType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut m: [&str; 2] = Default::default();
        s.split("/").enumerate().for_each(|(i, x)| m[i] = x);

        let super_type = MimeSuperType::from_str(m[0])?;

        let sub_type = match super_type {
            MimeSuperType::Application => {
                MimeSubType::Application(Application::from_str(m[1])?)
            }
            MimeSuperType::Audio => MimeSubType::Audio(Audio::from_str(m[1])?),
            MimeSuperType::Image => MimeSubType::Image(Image::from_str(m[1])?),
            MimeSuperType::Text => MimeSubType::Text(Text::from_str(m[1])?),
            MimeSuperType::Video => MimeSubType::Video(Video::from_str(m[1])?),
            _ => return Err(HeaderErrors::MimeTypeError(s.to_string()).into())
        };
        Ok(Self {
            super_type,
            sub_type
        })
    }
}

#[derive(Debug, EnumString)]
#[strum(serialize_all = "snake_case")]
#[strum(ascii_case_insensitive)]
pub enum MimeSuperType {
    Application,
    Audio,
    Font,
    Image,
    Model,
    Text,
    Video,
    Multipart,
    Unknown()
}

#[derive(Debug)]
pub enum MimeSubType {
    Application(Application),
    Audio(Audio),
    Image(Image),
    Text(Text),
    Video(Video)
}

#[derive(Debug, EnumString)]
#[strum(serialize_all = "snake_case")]
#[strum(ascii_case_insensitive)]
pub enum Application {
    Javascript,
    Json,
    Mp4,
    OctetStream,
    Zip
}

#[derive(Debug, EnumString)]
#[strum(serialize_all = "snake_case")]
#[strum(ascii_case_insensitive)]
pub enum Audio {
    XMidi,
    XWav
}

#[derive(Debug, EnumString)]
#[strum(serialize_all = "snake_case")]
#[strum(ascii_case_insensitive)]
pub enum Image {
    Bmp,
    Gif,
    Jpeg,
    Tiff
}

#[derive(Debug, EnumString)]
#[strum(serialize_all = "snake_case")]
#[strum(ascii_case_insensitive)]
pub enum Text {
    Html,
    Plain,
    Richtext,
    Css,
    Csv
}

#[derive(Debug, EnumString)]
#[strum(serialize_all = "snake_case")]
#[strum(ascii_case_insensitive)]
pub enum Video {
    Mpeg,
    Quicktime
}
