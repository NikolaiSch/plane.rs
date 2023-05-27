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

use {
    self::{
        super::errors,
        encoding::Encoding,
        locale::Locale,
        mime_types::MimeType
    },
    std::{
        error::Error,
        str::FromStr,
        string::ParseError
    }
};

#[derive(PartialEq, Eq, Debug)]
pub enum Header {
    UserAgent(String),
    AcceptLanguage(Locale),
    AcceptEncoding(Vec<Encoding>),
    Accept(Vec<MimeType>)
}

impl FromStr for Header {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (k, mut v) = s.split_once(":").unwrap();
        v = v.trim();
        match k.to_lowercase().trim() {
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
            "accept" => {
                let encoded_vec = s
                    .split(",")
                    .map(|x| {
                        return MimeType::from_header(
                            x.trim().split(";").next().unwrap()
                        )
                        .unwrap();
                    })
                    .collect();

                Ok(Header::Accept(encoded_vec))
            }
            _ => Err(errors::HeaderErrors::RequestError.into())
        }
    }
}
