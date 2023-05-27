use std::{
    collections::HashMap,
    io::Write,
    net::{
        Ipv4Addr,
        TcpListener
    },
    str::FromStr
};

use anyhow::Result;

use super::response::Httpify;
use crate::{
    config::config::{
        ServerConfig,
        ServerOpts
    },
    request::{
        headers::method::Method,
        request_parser::RequestParser
    },
    routing::route::{
        self,
        Handle,
        Route,
        RouteHandler,
        RouteMap
    }
};
pub struct Plane {
    pub config:   ServerConfig,
    pub handlers: RouteMap,

    pub tcp: Option<TcpListener>
}

impl Plane {
    /// Use this function to create a new instance of plane
    /// then, call helper methods on that (builder)
    pub fn board() -> Plane {
        return Plane {
            config:   ServerConfig::new(),
            handlers: HashMap::new(),
            tcp:      None
        };
    }

    pub fn route(
        &mut self,
        method: Method,
        path: &'static str,
        handler: RouteHandler
    ) -> Result<&mut Plane> {
        let route = Route::new(method, path.to_string());

        self.handlers.insert(route, handler);
        return Ok(self);
    }

    fn event_loop(&self) -> Result<()> {
        if let Some(x) = &self.tcp {
            for stream_res in x.incoming() {
                let mut stream = stream_res?;

                let mut req = RequestParser::new(stream.try_clone()?);
                req.parse()?;
                let res = self.handlers.execute_handler(&req.req).unwrap();

                for line in res.to_http() {
                    dbg!(&line);
                    let w = writeln!(stream, "{}", line.trim());
                    if let Err(_x) = w {
                        stream.flush().unwrap();
                    }
                }
            }
        }
        return Ok(());
    }

    pub fn takeoff(&mut self) -> () {
        self.tcp =
            Some(TcpListener::bind(self.config.get_full_addr()).unwrap());

        let _ = self.event_loop().unwrap();
    }
}

impl Default for Plane {
    fn default() -> Self { self::Plane::board() }
}
