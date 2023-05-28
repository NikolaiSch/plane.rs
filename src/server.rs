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
<<<<<<< HEAD
    tokio::{
        net::{
            TcpListener,
            TcpStream
        }
=======
    tokio::net::{
        TcpListener,
        TcpStream
>>>>>>> parent of 8fae1cc (i dont even know at this point)
    },
    tracing::{
        event,
        field::*,
        instrument,
<<<<<<< HEAD
        span,
        Level
    }
};
=======
        Level
    }
};
#[derive(Default)]
pub struct D {}
impl D {
    fn Default() -> _ {
        Route {
            method: Method::GET,
            path:   Uri(s)
        }
    }
}
>>>>>>> parent of 8fae1cc (i dont even know at this point)

pub struct Plane {
    pub config:   ServerConfig,
    pub handlers: RouteMap
}

impl Plane {
<<<<<<< HEAD
    /// Use this function to create a new instance of plane
    /// then, call helper methods on that (builder)
=======
    #[instrument(level = Level::DEBUG, skip_all)]
>>>>>>> parent of 8fae1cc (i dont even know at this point)
    pub fn board() -> Plane {
        Plane {
            config:   ServerConfig::new(),
            handlers: HashMap::new()
<<<<<<< HEAD
        }
    }

=======
        };
        event!(Level::DEBUG, "Boarding!");
        p
    }

    #[instrument(level = "TRACE", skip_all)]
>>>>>>> parent of 8fae1cc (i dont even know at this point)
    pub fn set(&mut self, opt: ServerOpts) -> Result<&mut Self> {
        let span = span!(Level::TRACE, "match_server_opts");
        let _enter = span.enter();
        match opt {
            ServerOpts::Host(host) => self.config.ip_addr = ServerConfig::parse_ip(host)?,
            ServerOpts::Port(port) => self.config.port = port,
            ServerOpts::Fallback(backup) => {
<<<<<<< HEAD
                let _ = self.handlers.insert(Route::Fallback, backup);
=======
                let _ = self.handlers.insert(D::Default(), backup);
>>>>>>> parent of 8fae1cc (i dont even know at this point)
            }
        };

        Ok(self)
    }

<<<<<<< HEAD
    pub fn route(&mut self, method: Method, path: &str, handler: RouteHandler) -> Result<&mut Plane> {
        let route = Route::new(method, Uri::from_str(path)?);

=======
    #[instrument(level = "INFO", "New Route", skip_all)]
    pub fn route(&mut self, route: Route, handler: RouteHandler) -> Result<&mut Plane> {
        let route = Route::new(Method::GET, Uri::from_str("/")?);
>>>>>>> parent of 8fae1cc (i dont even know at this point)
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
        self.event_loop().await
    }
}
