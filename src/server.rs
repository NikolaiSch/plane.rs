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
        method,
        Method,
        Uri
    },
    std::{
        self,
        borrow::BorrowMut,
        collections::HashMap,
        path,
        str::FromStr
    },
    tokio::net::{
        TcpListener,
        TcpStream
    },
    tracing::{
        event,
        field::*,
        instrument,
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

pub struct Plane {
    pub config:   ServerConfig,
    pub handlers: RouteMap
}

impl Plane {
    #[instrument(level = Level::DEBUG, skip_all)]
    pub fn board() -> Plane {
        let p = Plane {
            config:   ServerConfig::new(),
            handlers: HashMap::new()
        };
        event!(Level::DEBUG, "Boarding!");
        p
    }

    #[instrument(level = "TRACE", skip_all)]
    pub fn set(&mut self, opt: ServerOpts) -> Result<&mut Self> {
        match opt {
            ServerOpts::Host(host) => self.config.ip_addr = ServerConfig::parse_ip(host)?,
            ServerOpts::Port(port) => self.config.port = port,
            ServerOpts::Fallback(backup) => {
                let _ = self.handlers.insert(D::Default(), backup);
            }
        };

        Ok(self)
    }

    #[instrument(level = "INFO", "New Route", skip_all)]
    pub fn route(&mut self, route: Route, handler: RouteHandler) -> Result<&mut Plane> {
        let route = Route::new(Method::GET, Uri::from_str("/")?);
        self.handlers.insert(route, handler);

        event!(Level::INFO, "New Route! {} {}", Uri::from_str("/")?, Method::GET);

        Ok(self)
    }

    #[instrument(level = "DEBUG", "Connection Handler", skip_all, ret, err)]
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
