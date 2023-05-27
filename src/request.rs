use {
    anyhow::{
        anyhow,
        bail,
        Result
    },
    http::{
        request::Builder,
        HeaderName,
        HeaderValue,
        Request,
        Version
    },
    std::{
        io::{
            BufRead,
            BufReader
        },
        net::TcpStream,
        str::FromStr
    }
};

pub struct IncomingRequest {
    req:    Builder,
    reader: Vec<String>
}

impl IncomingRequest {
    pub fn new(stream: TcpStream) -> Self {
        Self {
            req:    Request::builder(),
            reader: BufReader::new(stream)
                .lines()
                .map(|x| x.unwrap())
                .take_while(|x| !x.is_empty())
                .collect()
        }
    }

    fn parse(&mut self) -> Result<&mut Self> {
        if let Some((&first_line, rest)) = self.reader.split_first() {
            let req = UTF8Request {
                first_line,
                rest: rest.iter().collect()
            };

            req.parse_first_line(self)?;
            req.parse_headers(self)?;

            Ok(self)
        } else {
            bail!("Malformed Request: Empty Request");
        }
    }
}

struct UTF8Request {
    first_line: String,
    rest:       Vec<String>
}

impl UTF8Request {
    pub fn parse_first_line(&self, req: &mut IncomingRequest) -> Result<()> {
        for (i, x) in self.first_line.split(" ").enumerate() {
            match i {
                0 => Ok(req.req.method(x)),
                1 => Ok(req.req.uri(x)),
                2 => Ok(req.req.version(Version::http_11)),
                _ => bail!("Malformed Request: Invalid First Line of Request")
            };
        }

        Ok(())
    }

    pub fn parse_headers(&self, req: &mut IncomingRequest) -> Result<()> {
        self.rest
            .iter()
            .map(|x| Self::parse_header(x)?)
            .map(|(n, v)| req.req.header(n, v))
            .collect();

        Ok(())
    }

    fn parse_header(header: &str) -> Result<(HeaderName, HeaderValue)> {
        let parts: Vec<&str> = header.split(":").collect();

        if let Some((&f, &s)) = parts.split_first() {
            let name = HeaderName::from_str(f)?;
            let val = HeaderValue::from_str(s.get(0).unwrap())?;

            return Ok((name, val));
        } else {
            bail!("Malformed Request: Incorrect Header Format")
        }
    }
}
