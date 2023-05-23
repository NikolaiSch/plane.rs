use std::{
    io::{
        BufRead,
        BufReader,
        Read
    },
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

pub struct RequestParser<'a> {
    input: BufReader<&'a mut TcpStream>
}

impl<'a> RequestParser<'_> {
    pub fn new(stream: &'a mut TcpStream) -> RequestParser<'a> {
        let reader = BufReader::new(stream);
        dbg!("reading complete");
        RequestParser { input: reader }
    }

    pub fn parse_stream(&mut self) -> Result<()> {
        let mut first_line = binding.split(" ");
        match first_line.next().unwrap() {
            "GET" => self.data.method = Method::GET,
            "POST" => self.data.method = Method::POST,
            _ => panic!("malformed req: e1")
        }

        self.data.route = first_line.next().unwrap().to_string();

        while let Some(line) = self.next() {
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

    fn first_line(&self) -> Request {
    }
}

impl Iterator for RequestParser<'_> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let mut buf = String::new();
        let recieved = self.input.read_line(&mut buf);

        match recieved {
            Ok(consumed) => {
                self.input.consume(consumed);
                Some(buf)
            }
            Err(x) => None
        }
    }
}
