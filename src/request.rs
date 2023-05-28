use {
    crate::{
        error::RequestError,
        Req
    },
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
        default::default,
        str::FromStr
    },
    tokio::{
        io::AsyncBufReadExt,
        join
    }
};

pub enum Opts {
    Method(Method),
    Uri(Uri),
    Version(Version),
    Header((HeaderName, HeaderValue))
}

#[derive(Debug, Default)]
pub struct IncomingRequest {
    pub data:   Req,
    pub reader: Vec<String>
}

impl IncomingRequest {
    pub async fn new(stream: tokio::net::tcp::OwnedReadHalf) -> Result<Self> {
        let mut lines = tokio::io::BufReader::new(stream).lines();
        let mut v = vec![];
        while let Some(x) = lines.next_line().await? {
            v.push(x);
        }
        let mut s = Self {
            reader: v,
            data:   Request::new(vec![])
        };

        s.parse().await?;

        return Ok(s);
    }

    async fn parse(&mut self) -> Result<()> {
        let mut req = UTF8Request::new(self)?;

        let mut opts = req.parse_first_line()?;
        let mut opts2 = &mut (req.parse_headers()?.into_iter().map(Opts::Header).collect());

        tokio::join!(opts, opts2).await;

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
    pub fn new(s: &mut IncomingRequest) -> anyhow::Result<UTF8Request> {
        if let Some((first_line, rest)) = s.reader.split_first() {
            return Ok(UTF8Request {
                first_line: first_line.to_string(),
                rest:       rest.iter().map(|f| f.to_string()).collect()
            });
        } else {
            bail!(RequestError::EmptyRequest);
        }
    }

    pub async fn parse_first_line(&mut self) -> Result<Vec<Opts>> {
        let opts = self
            .first_line
            .split(' ')
            .enumerate()
            .map(|(i, x)| {
                match i {
                    0 => Opts::Method(Method::from_str(x).unwrap()),
                    1 => Opts::Uri(Uri::from_str(x).unwrap()),
                    2 => Opts::Version(Version::HTTP_11),
                    _ => panic!("should be unreachable")
                }
            })
            .collect();

        Ok(opts)
    }

    pub async fn parse_headers(&self) -> Result<Vec<(HeaderName, HeaderValue)>> {
        let i = self.rest.iter();
        let mut v = vec![];
        for x in i {
            v.push(tokio::spawn(async move { Self::parse_header(x).await }))
        }
        let mut a = vec![];
        for i in v.array_chunks::<5>() {
            join!(.. i)
        }

        Ok(i)
    }

    async fn parse_header(header: &str) -> Result<(HeaderName, HeaderValue)> {
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
mod incoming_request {
    use {
        super::*,
        http::Method,
        std::fs::File
    };

    const REQ_PATH: &str = "test_data/requests/req2.txt";

    fn new_incoming(path: &str) -> Result<IncomingRequest> {
        let file = File::open(path)?;
        Ok(IncomingRequest::new(file)?)
    }

    #[test]
    fn new_incoming_request() -> Result<()> {
        let ireq = new_incoming(REQ_PATH)?;

        assert_eq!(ireq.data.method(), &Method::GET);
        assert_eq!(ireq.data.version(), Version::HTTP_11);
        assert_eq!(*ireq.data.uri(), *"/");

        dbg!(ireq.data.headers());

        Ok(())
    }

    // #[test]
    // fn
}
