use std::{
    io::Read,
    net::TcpStream
};

use anyhow::Result;

use crate::enums::request_opts::{
    Encoding,
    Header,
    Headers,
    Method
};

pub struct Parser {
    stream: TcpStream,

    data: ParsedData
}

pub struct ParsedData {
    method: Method,
    route:  String,

    headers: Headers
}

impl Default for ParsedData {
    fn default() -> Self {
        return ParsedData {
            method:  Method::UNSET,
            route:   "".to_string(),
            headers: vec![]
        };
    }
}

impl Parser {
    pub fn new(stream: TcpStream) -> Parser {
        Parser {
            stream,
            data: ParsedData::default()
        }
    }

    pub fn parse_stream(&mut self) -> Result<()> {
        let mut buffer = String::new();
        self.stream
            .read_to_string(&mut buffer)?;
        let mut lines = buffer.lines();

        let mut first_line = lines.next().unwrap().split(" ");
        match first_line.next().unwrap() {
            "GET" => self.data.method = Method::GET,
            "POST" => self.data.method = Method::POST,
            _ => panic!("malformed req: e1")
        }

        self.data.route = first_line.next().unwrap().to_string();

        while let Some(line) = lines.next() {
            if let Some((key, val)) = line.split_once(":") {
                match key {
                    "User-Agent" => {
                        self.data
                            .headers
                            .push(Header::UserAgent(val.trim().to_string()));
                    }
                    "Accept-Encoding" => {
                        let mut encodings = vec![];

                        let encoded_vec = val
                            .split(",")
                            .map(|x| x.trim().parse::<Encoding>().unwrap());

                        for v in encoded_vec {
                            encodings.push(v);
                        }

                        self.data
                            .headers
                            .push(Header::AcceptEncoding(encodings))
                    }
                    _ => {}
                }
            }
        }

        todo!()
    }
}
