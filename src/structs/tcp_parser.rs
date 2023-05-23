use std::{
    io::{
        BufRead,
        BufReader,
        Lines,
        Read
    },
    net::TcpStream,
    ops::Deref,
    str::FromStr
};

use anyhow::Result;

use super::request::Request;
use crate::enums::request_opts::{
    Encoding,
    Header,
    Locale,
    Method,
    HTTP
};

// type StringLines = ;
pub struct RequestParser {
    inner:   Vec<String>,
    pub req: Request
}

impl<'a> RequestParser {
    pub fn new(mut stream: TcpStream) -> Self {
        let reader = BufReader::new(&mut stream);
        let lines = reader
            .lines()
            .map(|x| x.unwrap())
            .take_while(|line| !line.is_empty())
            .collect();

        dbg!(&lines);

        RequestParser {
            inner: lines,
            req:   Default::default()
        }
    }

    pub fn parse(&mut self) -> Result<()> {
        self.parse_first_line()?;
        self.req.client.headers = self.parse_headers()?;
        dbg!(&self.req);
        Ok(())
    }

    fn parse_first_line<'b>(&mut self) -> Result<()> {
        let first_line = self.next().unwrap();
        dbg!(&first_line);
        let mut s = first_line.split(" ");
        self.req.method = Method::from_str(s.next().unwrap()).unwrap();
        self.req.route = s.next().unwrap().to_string();
        self.req.client.http_version =
            HTTP::from_str(s.next().unwrap()).unwrap();

        Ok(())
    }

    fn parse_headers<'b>(&mut self) -> Result<Vec<Header>> {
        let mut v = vec![];
        for line in self {
            if let Some((key, val)) = line.split_once(":") {
                let header = RequestParser::match_header(key, val);
                if let Some(x) = header {
                    v.push(x);
                }
            }
        }
        dbg!(1);
        Ok(v)
    }

    fn match_header(key: &str, val: &str) -> Option<Header> {
        let k = &*key.trim().to_lowercase();
        let v = val.trim();

        match k {
            "user-agent" => Some(Header::UserAgent(v.to_string())),
            "accept-encoding" => {
                let encoded_vec = val
                    .split(",")
                    .map(|x| x.trim().parse::<Encoding>().unwrap())
                    .collect();

                Some(Header::AcceptEncoding(encoded_vec))
            }
            "accept-language" => {
                Some(Header::AcceptLanguage(Locale::from_str(v).unwrap()))
            }
            _ => None
        }
    }
}

impl Iterator for RequestParser {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let s = self.inner.split_first().clone();
        if let Some((current, rest)) = s {
            let c = current.clone();
            self.inner = rest.clone().to_vec();
            Some(c)
        } else {
            None
        }
    }
}
