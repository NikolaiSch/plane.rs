use std::{
    fmt::Display,
    str::FromStr
};

use crate::request::errors::HeaderErrors;

#[derive(Debug, PartialEq, Eq)]
pub enum HTTPVersion {
    Unassigned,

    V1_0,
    V1_1,
    V2,
    V3
}

impl FromStr for HTTPVersion {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "HTTP/1.0" => Ok(HTTPVersion::V1_0),
            "HTTP/1.1" => Ok(HTTPVersion::V1_1),
            "HTTP/2" => Ok(HTTPVersion::V2),
            "HTTP/3" => Ok(HTTPVersion::V3),
            x => Err(HeaderErrors::HTTPVersionError(x.to_string()).into())
        }
    }
}

impl Display for HTTPVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            HTTPVersion::V1_0 => "HTTP/1.0",
            HTTPVersion::V1_1 => "HTTP/1.1",
            HTTPVersion::V2 => "HTTP/2",
            HTTPVersion::V3 => "HTTP/3",

            HTTPVersion::Unassigned => return Err(std::fmt::Error)
        };

        f.write_str(s)
    }
}
