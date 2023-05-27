use {
    crate::Req,
    anyhow::{
        bail,
        Result
    },
    http::{
        HeaderName,
        HeaderValue,
        Method,
        Request,
        Uri,
        Version
    },
    std::{
        io::{
            BufRead,
            BufReader,
            Read
        },
        str::FromStr
    }
};

pub enum Opts {
    Method(Method),
    Uri(Uri),
    Version(Version),
    Header((HeaderName, HeaderValue))
}

pub struct IncomingRequest {
    pub data:   Req,
    pub reader: Vec<String>
}

impl IncomingRequest {
    pub fn new(stream: impl Read) -> Self {
        Self {
            reader: BufReader::new(stream)
                .lines()
                .map(|x| x.unwrap())
                .take_while(|x| !x.is_empty())
                .collect(),
            data:   Request::new(vec!["Working".to_string()])
        }
    }

    pub fn parse(&mut self) -> Result<()> {
        let mut req = if let Some((first_line, rest)) = self.reader.split_first() {
            UTF8Request {
                first_line: first_line.to_string(),
                rest:       rest.iter().map(|f| f.to_string()).collect()
            }
        } else {
            bail!("Malformed Request: Empty Request");
        };

        let mut opts = req.parse_first_line()?;

        opts.append(
            &mut (req
                .parse_headers()?
                .into_iter()
                .map(Opts::Header)
                .collect())
        );

        for i in opts.into_iter() {
            match i {
                Opts::Uri(uri) => *self.data.uri_mut() = uri,
                Opts::Method(method) => *self.data.method_mut() = method,
                Opts::Version(version) => *self.data.version_mut() = version,
                Opts::Header((name, value)) => {
                    self.data.headers_mut().insert(name, value);
                }
            }
        }

        Ok(())
    }
}

impl From<IncomingRequest> for Request<Vec<String>> {
    fn from(val: IncomingRequest) -> Self {
        val.data
    }
}

struct UTF8Request {
    first_line: String,
    rest:       Vec<String>
}

impl UTF8Request {
    pub fn parse_first_line(&mut self) -> Result<Vec<Opts>> {
        let opts = self
            .first_line
            .split(' ')
            .enumerate()
            .map(|(i, x)| {
                match i {
                    0 => Opts::Method(Method::from_str(x).unwrap()),
                    1 => Opts::Uri(Uri::from_str(x).unwrap()),
                    2 => Opts::Version(Version::HTTP_11),
                    _ => unreachable!()
                }
            })
            .collect();

        Ok(opts)
    }

    pub fn parse_headers(&self) -> Result<Vec<(HeaderName, HeaderValue)>> {
        self.rest.iter().map(|x| Self::parse_header(x)).collect()
    }

    fn parse_header(header: &str) -> Result<(HeaderName, HeaderValue)> {
        let parts: Vec<&str> = header.split(':').collect();

        if let Some((&f, s)) = parts.split_first() {
            let name = HeaderName::from_str(f)?;
            let val = HeaderValue::from_str(s.first().unwrap())?;

            Ok((name, val))
        } else {
            bail!("Malformed Request: Incorrect Header Format")
        }
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        http::Method,
        std::fs::File
    };

    const REQ_PATH: &str = "test_data/requests/req2.txt";

    #[test]
    fn new_incoming_request() -> Result<()> {
        let file = File::open(REQ_PATH)?;
        let mut ireq = IncomingRequest::new(file);

        ireq.parse()?;

        assert_eq!(ireq.data.method(), &Method::GET);
        assert_eq!(ireq.data.version(), Version::HTTP_11);
        assert_eq!(*ireq.data.uri(), *"/");

        Ok(())
    }
}
