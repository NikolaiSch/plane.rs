use std::{
    error::Error,
    str::FromStr,
    string::ParseError
};

use self::{
    encoding::Encoding,
    locale::Locale,
    mime_types::MimeType
};
use super::errors;

pub mod encoding;
pub mod http_version;
pub mod locale;
pub mod method;
pub mod mime_types;

#[cfg(test)]
mod encoding_test;

#[cfg(test)]
mod http_version_test;

#[cfg(test)]
mod locale_test;

#[cfg(test)]
mod method_test;

#[cfg(test)]
mod mime_types_test;

pub enum Header {
    UserAgent(String),
    AcceptLanguage(Locale),
    AcceptEncoding(Vec<Encoding>),
    Accept(Vec<MimeType>)
}

impl FromStr for Header {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (k, v) = s.split_once(":").unwrap();
        match k {
            "user-agent" => Ok(Header::UserAgent(v.to_string())),
            "accept-encoding" => {
                let encoded_vec = v
                    .split(",")
                    .map(|x| x.trim().parse::<Encoding>().unwrap())
                    .collect();

                Ok(Header::AcceptEncoding(encoded_vec))
            }
            "accept-language" => {
                Ok(Header::AcceptLanguage(Locale::from_str(v).unwrap()))
            }
            _ => Err(errors::HeaderErrors::RequestError.into())
        }
    }
}
