use std::iter;

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
        io::{
            BufRead,
            BufReader,
            BufWriter,
            Read,
            Write
        },
        str::FromStr
    },
    tokio::{
        self,
        task::JoinSet
    },
    tracing::{
        instrument,
        log::error,
        trace
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
    const R: std::result::Result<IncomingRequest, anyhow::Error> = ;
    pub async fn new(stream: impl Read + BufRead) -> Result<Self> {
        let mut lines: BufReader<R>;
        let v: Vec<String> = Vec::new();
        let mut s: String = String::new();

        let mut s = Self {
            reader: Vec::new(),
            data:   Request::new(vec![])
        };

        self.

        Ok(Self { ..default() }) 
    }
        
    }

    #[instrument(skip())]
    async fn parse() -> Result<()> {
        let mut req = UTF8Request::new(&mut )?;
        trace!("made new UTF8Request");
        let mut opts = req.parse_first_line().await?;
        let mut opts2: Vec<Opts> = req.parse_headers().await?.into_iter().map(Opts::Header).collect();
        trace!("parsed all headers to enum");
        opts.append(&mut opts2);

        for i in opts {
            match i {
                Opts::Uri(uri) => *.data.uri_mut() = uri,
                Opts::Method(method) => *.data.method_mut() = method,
                Opts::Version(version) => *.data.version_mut() = version,
                Opts::Header((name, value)) => {
                    .data.headers_mut().insert(name, value);
                }
            }
        }

        trace!("parsed all enums, and put them in container");

        Ok(())
    }

  impl From<IncomingRequest> for Request<Vec<String>> {
    fn from(val: IncomingRequest) -> Self {
        val.data
    }
}

#[derive(Debug)]
struct UTF8Request {
    first_line: String,
    rest:       Vec<String>
}

impl UTF8Request {
    #[instrument(fields(ret), skip(s))]
    pub fn new(s: &mut IncomingRequest) -> anyhow::Result<UTF8Request> {
        if let Some((first_line, rest)) = s.reader.split_first() {
            let x = Ok(UTF8Request {
                first_line: first_line.to_string(),
                rest:       rest.iter().map(|f| f.to_string()).collect()
            });
            trace!("Successfully split lines to UTF8Request");
            return x;
        } else {
            error!("error, nearlt empty request");
            bail!(RequestError::EmptyRequest);
        }
    }

    #[instrument(fields(ret), skip(self))]
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

    #[instrument(fields(err, ret))]
    pub async fn parse_headers(&self) -> Result<Vec<(HeaderName, HeaderValue)>> {
        let i = self.rest.clone().into_iter();
        let mut v = JoinSet::new();
        for x in i {
            v.spawn(async move { Self::parse_header(&x).await });
        }

        let mut a = vec![];

        while let Some(Ok(x)) = v.join_next().await {
            a.push(x?);
        }

        Ok(a)
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
        std::ptr::write_volatile,
        tokio::net::{
            tcp::OwnedWriteHalf,
            TcpListener,
            TcpStream
        }
    };

    const REQ_PATH: &str = "test_data/requests/req2.txt";

    async fn new_incoming() -> Result<OwnedWriteHalf> {
        let mut stream = TcpStream::connect("127.0.0.1:8080")
            .await?
            .into_split()
            .1
            .writable()
            .await?;

        bail!("This is not good enough")
    }
}
