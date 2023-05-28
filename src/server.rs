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
        match opt {
            ServerOpts::Host(host) => self.config.ip_addr = ServerConfig::parse_ip(host)?,
            ServerOpts::Port(port) => self.config.port = port,
            ServerOpts::Fallback(backup) => {
                let _ = self.handlers.insert(Route::Fallback, backup);
            }
        };

        Ok(self)
    }

    pub fn route(
        &mut self,
        method: Method,
        path: &str,
        handler: RouteHandler
    ) -> Result<&mut Plane> {
        let route = Route::new(method, Uri::from_str(path)?);

        self.handlers.insert(route, handler);
        Ok(self)
    }

    async fn conn_handler(&self, conn: TcpStream) -> anyhow::Result<()> {
        let (read, write) = conn.into_split();
        let ireq = IncomingRequest::new(read)?;

        let mut res = self.handlers.execute_handler(&ireq.into())?;

        res.write_to_stream(conn)?;

        Ok(())
    }

    async fn event_loop(&self) -> anyhow::Result<()> {
        let listener = TcpListener::bind(self.config.get_socket_addr()).await?;
        loop {
            for (conn, socket) in listener.accept().await {
                self.conn_handler(conn).await;
            }
        }
        Ok(())
    }

    pub async fn takeoff(&mut self) -> Result<()> {
        self.event_loop().await?
    }
}
