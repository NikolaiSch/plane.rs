use std::str::FromStr;

use super::super::errors::HeaderErrors;

#[derive(Debug, Hash, PartialEq, Eq)]
pub enum Encoding {
    Unassigned,

    Gzip,
    Compress,
    Deflate,
    Br,
    Identity,
    Asterix
}

impl FromStr for Encoding {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &*s.trim().to_lowercase() {
            "gzip" => Ok(Self::Gzip),
            "compress" => Ok(Self::Compress),
            "deflate" => Ok(Self::Deflate),
            "br" => Ok(Self::Br),
            "identity" => Ok(Self::Identity),
            "*" => Ok(Self::Asterix),
            x => Err(HeaderErrors::EncodingError(x.to_string()).into())
        }
    }
}
