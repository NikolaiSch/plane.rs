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
    std::str::FromStr,
    tokio::{
        io::AsyncBufReadExt,
        task::JoinSet
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

        let mut opts = req.parse_first_line().await?;
        let mut opts2: Vec<Opts> = req.parse_headers().await?.into_iter().map(Opts::Header).collect();

        opts.append(&mut opts2);

        for i in opts {
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

    pub fn to_vec(&mut self) -> anyhow::Result<Vec<String>> {
        Ok(self
            .data
            .headers_mut()
            .iter_mut()
            .enumerate()
            .map(|(_x, (n, v))| format!("{}: {}", n, v.to_str().unwrap()))
            .collect())
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
            let name = HeaderName::from_str(dbg!(f))?;
            let val = HeaderValue::from_str(dbg!(s.first()).unwrap())?;

            Ok((name, val))
        } else {
            bail!("Malformed Request: Incorrect Header Format")
        }
    }
}

// #[cfg(test)]
// mod incoming_request {
//     use {
//         super::*,
//         http::Method,
//         std::fs::File,
//         tokio::net::{
//             tcp::OwnedWriteHalf,
//             TcpListener,
//             TcpStream
//         }
//     };

//     const REQ_PATH: &str = "test_data/requests/req2.txt";

//     async fn new_incoming() -> Result<OwnedWriteHalf> {
//         let (listener, _addr) =
// TcpListener::bind("127.0.0.1:7574").await?.accept().await?;         let
// (read, write) = TcpStream::into_split(listener);

//         let d = IncomingRequest::new(read).await?;

//         Ok(write)
//     }
// }
