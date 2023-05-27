use std::str::FromStr;

use super::errors::HeaderErrors;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug)]
pub enum MimeType {
    Unassigned,

    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
    OPTIONS,
    HEAD
}

impl FromStr for MimeType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(MimeType::GET),
            "POST" => Ok(MimeType::POST),
            "PUT" => Ok(MimeType::PUT),
            "PATCH" => Ok(MimeType::PATCH),
            "DELETE" => Ok(MimeType::DELETE),
            "OPTIONS" => Ok(MimeType::OPTIONS),
            "HEAD" => Ok(MimeType::HEAD),
            x => Err(HeaderErrors::MethodError(x.to_string()).into())
        }
    }
}
