use std::{
    io::{
        BufRead,
        BufReader
    },
    net::TcpStream,
    str::FromStr
};

use anyhow::Result;

use super::headers::{
    http_version::HTTPVersion,
    method::MimeType,
    Header
};

pub struct Client {
    pub headers:      Vec<Header>,
    pub http_version: HTTPVersion
}

pub struct Request {
    pub client: Client,

    pub method: MimeType,
    pub route:  String
}

impl Default for Request {
    fn default() -> Self {
        return Request {
            client: Client {
                http_version: HTTPVersion::Unassigned,
                headers:      vec![]
            },
            method: MimeType::Unassigned,
            route:  "/".to_string()
        };
    }
}

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

        RequestParser {
            inner: lines,
            req:   Default::default()
        }
    }

    pub fn parse(&mut self) -> Result<()> {
        self.parse_first_line()?;
        self.req.client.headers = self.parse_headers()?;
        Ok(())
    }

    fn parse_first_line<'b>(&mut self) -> Result<()> {
        let first_line = self.next().unwrap();
        dbg!(&first_line);
        let mut s = first_line.split(" ");
        self.req.method = MimeType::from_str(s.next().unwrap()).unwrap();
        self.req.route = s.next().unwrap().to_string();
        self.req.client.http_version =
            HTTPVersion::from_str(s.next().unwrap()).unwrap();

        Ok(())
    }

    fn parse_headers<'b>(&mut self) -> Result<Vec<Header>> {
        let mut v = vec![];
        for line in self {
            let header = Header::from_str(&line)?;
            v.push(header);
        }
        Ok(v)
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
