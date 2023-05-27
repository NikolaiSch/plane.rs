use {
    crate::{
        request::IncomingRequest,
        route::{
            Handle,
            Route,
            RouteMap
        },
        server_config::ServerConfig,
        RouteHandler
    },
    anyhow::Result,
    http::{
        response::Builder,
        Method,
        Uri
    },
    std::{
        collections::HashMap,
        io::Write,
        net::TcpListener,
        str::FromStr
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
        return Plane {
            config:   ServerConfig::new(),
            handlers: HashMap::new()
        };
    }

    pub fn route(
        &mut self,
        method: Method,
        path: &str,
        handler: RouteHandler
    ) -> Result<&mut Plane> {
        let route = Route::new(method, Uri::from_str(path)?);

        self.handlers.insert(route, handler);
        return Ok(self);
    }

    fn event_loop(&self) -> Result<()> {
        let listener = TcpListener::bind(self.config.get_socket_addr())?;

        for conn in listener.incoming() {
            let mut stream = conn?;

            let mut ireq = IncomingRequest::new(stream.try_clone()?);
            ireq.parse()?;

            let res = self.handlers.execute_handler(&ireq.try_into()?).unwrap();

            let res = Builder::new();

            // for line in res.to_http() {
            //     dbg!(&line);
            //     let w = writeln!(stream, "{}", line.trim());
            //     if let Err(_x) = w {
            //         stream.flush().unwrap();
            //     }
            // }
        }

        return Ok(());
    }

    pub fn takeoff(&mut self) -> () {
        self.event_loop().unwrap();
    }
}
