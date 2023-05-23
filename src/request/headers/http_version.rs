use std::{
    fmt::Display,
    str::FromStr
};

#[derive(Debug)]
pub enum HTTPStatus {
    Unassigned,

    V1_0,
    V1_1,
    V2,
    V3
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
