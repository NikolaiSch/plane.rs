use std::{
    io::Read,
    net::TcpStream,
    str::FromStr
};

use anyhow::Result;

use super::request::Request;
use crate::enums::request_opts::{
    Encoding,
    Header,
    Locale,
    Method
};

pub struct Parser {
    stream: TcpStream,

    pub data: Request
}

impl Parser {
    pub fn new(stream: TcpStream) -> Parser {
        Parser {
            stream,
            data: Request::default()
        }
    }

    pub fn parse_stream(&mut self) -> Result<()> {
        let mut buffer = String::new();
        self.stream
            .read_to_string(&mut buffer)?;
        let mut lines = buffer.lines();

        let mut first_line = lines.next().unwrap().split(" ");
        match first_line.next().unwrap() {
            "GET" => self.data.method = (Method::GET),
            "POST" => self.data.method = (Method::POST),
            _ => panic!("malformed req: e1")
        }

        self.data.route = first_line.next().unwrap().to_string();

        while let Some(line) = lines.next() {
            if let Some((key, mut val)) = line.split_once(":") {
                val = val.trim();
                let header = match key {
                    "User-Agent" => {
                        Some(Header::UserAgent(val.trim().to_string()))
                    }
                    "Accept-Encoding" => {
                        let mut encodings = vec![];

                        let encoded_vec = val
                            .split(",")
                            .map(|x| x.trim().parse::<Encoding>().unwrap());

                        for v in encoded_vec {
                            encodings.push(v);
                        }

                        Some(Header::AcceptEncoding(encodings))
                    }
                    "Accept-Language" => {
                        Some(Header::AcceptLanguage(
                            Locale::from_str(val).unwrap()
                        ))
                    }
                    _ => None
                };

                if let Some(x) = header {
                    self.data.client.headers.push(x)
                }
            }
        }
        Ok(())
    }
}
