use {
    anyhow::Result,
    http::{
        request::Builder,
        Method,
        Request
    },
    std::{
        io::{
            BufRead,
            BufReader
        },
        net::TcpStream
    }
};

pub struct IncomingRequest {
    req:    Builder,
    reader: Vec<String>
}

impl IncomingRequest {
    fn new(stream: TcpStream) -> Self {
        Self {
            req:    Request::builder(),
            reader: BufReader::new(stream)
                .lines()
                .(predicate)
                .map(|x| x.unwrap())
        }
    }

    fn parse(&mut self) -> Result<Self> {
    }
}

struct UTF8Request {
    first_line: String,
    rest:       Vec<String>
}
