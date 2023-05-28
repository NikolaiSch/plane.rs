use {
    crate::{
        body_parser::ToHTTP,
        request::IncomingRequest,
        route::{
            Handle,
            Route,
            RouteMap
        },
        server_config::{
            ServerConfig,
            ServerOpts
        },
        RouteHandler
    },
    anyhow::Result,
    http::{
        Method,
        Uri
    },
    std::{
        self,
        borrow::BorrowMut,
        collections::HashMap,
        str::FromStr
    },
    tokio::{
        io::{
            AsyncBufRead,
            AsyncBufReadExt,
            AsyncRead,
            AsyncReadExt,
            AsyncWrite,
            AsyncWriteExt
        },
        net::{
            tcp::{
                OwnedReadHalf,
                OwnedWriteHalf
            },
            TcpListener,
            TcpStream
        }
    },
    tracing::{
        event,
        field::*,
        info,
        info_span,
        instrument,
        span,
        warn,
        warn_span,
        Level
    }
};

pub struct Plane {
    pub config:   ServerConfig,
    pub handlers: RouteMap
}

impl Plane {
    /// Use this function to create a new instance of plane
    /// then, call helper methods on that (builder)
    pub fn board() -> Plane {
        Plane {
            config:   ServerConfig::new(),
            handlers: HashMap::new()
        }
    }

    pub fn set(&mut self, opt: ServerOpts) -> Result<&mut Self> {
        let span = span!(Level::TRACE, "match_server_opts");
        let _enter = span.enter();
        match opt {
            ServerOpts::Host(host) => self.config.ip_addr = ServerConfig::parse_ip(host)?,
            ServerOpts::Port(port) => self.config.port = port,
            ServerOpts::Fallback(backup) => {
                let _ = self.handlers.insert(Route::Fallback, backup);
            }
        };

        Ok(self)
    }

    pub fn route(&mut self, method: Method, path: &str, handler: RouteHandler) -> Result<&mut Plane> {
        let route = Route::new(method, Uri::from_str(path)?);

        self.handlers.insert(route, handler);
        Ok(self)
    }

    #[instrument(level = "INFO", "Connection Handler", skip_all)]
    async fn conn_handler(&self, conn: TcpStream) -> anyhow::Result<()> {
        let (read, mut write) = conn.into_split();
        event!(Level::TRACE, "Successfully split streams");

        let ireq = IncomingRequest::new(read).await?;
        event!(
            Level::TRACE,
            "Created and parsed the IncomingRequest the Tcp Server"
        );

        let mut res = self.handlers.execute_handler(&ireq.into())?;
        event!(Level::TRACE, "Created and parsed an IncomingRequest from stream");

        res.write_to_stream(write.borrow_mut()).await?;
        event!(Level::INFO, "Successfully Wrote the response to the client");

        Ok(())
    }

    async fn event_loop(&self) -> anyhow::Result<()> {
        let listener = TcpListener::bind(self.config.get_socket_addr()).await?;
        loop {
            if let Ok((conn, _socket)) = listener.accept().await {
                self.conn_handler(conn).await?;
            }
        }
    }

    #[instrument(level = Level::INFO, fields(config = Empty), skip_all)]
    pub async fn takeoff(&mut self) -> Result<()> {
        Ok(self.event_loop().await?)
    }
}
