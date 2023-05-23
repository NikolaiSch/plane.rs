use std::{
    collections::HashMap,
    io::{
        Read,
        Write
    },
    net::TcpListener
};

use anyhow::Result;

use super::{
    config::ServerConfig,
    route::{
        Route,
        RouteHandler
    },
    server::Plane,
    tcp_parser
};
use crate::{
    enums::{
        config::ServerOpts,
        ip::IPType,
        request_opts::Method
    },
    prelude::Response,
    structs::response::Httpify,
    traits::validate::{
        Validate,
        ValidationResult
    }
};

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

    pub fn set(&mut self, opt: ServerOpts) -> Result<&mut Plane> {
        match opt {
            ServerOpts::Host(ip) => {
                self.config.ip_addr = ip.parse::<IPType>()?;
            }
            ServerOpts::Port(port) => {
                self.config.port = port;
            }
            ServerOpts::Fallback(Some(route)) => {
                self.config.fallback = Some(route);
            }
            ServerOpts::Fallback(None) => {
                self.config.fallback = None;
            }
        }
        Ok(self)
    }

    pub fn route(
        &mut self,
        method: Method,
        path: &'static str,
        route: RouteHandler
    ) -> Result<&mut Plane> {
        let hash_route = Route::new(method, path.to_string());
        let route = hash_route.to_route(route);

        self.handlers.insert(hash_route, route);

        return Ok(self);
    }

    fn event_loop(&self) -> Result<()> {
        if let Some(x) = &self.tcp {
            for stream_res in x.incoming() {
                let mut stream = stream_res?;
                let mut parser = tcp_parser::Parser::new(&mut stream);
                parser.parse_stream()?;

                let hr = Route::new(
                    parser.data.method.clone(),
                    parser.data.route.clone()
                );

                let route = self.handlers.get(&hr).unwrap();

                let res = (route.handler)(&parser.data);

                for line in res.to_http() {
                    let w = writeln!(stream, "{}", line);
                    if let Err(x) = w {
                        stream.flush().unwrap();
                    }
                }
            }
        }
        return Ok(());
    }

    pub fn takeoff(&mut self) -> () {
        if let ValidationResult::Error(x) = self.config.validate() {
            panic!("{x}")
        }

        self.tcp =
            Some(TcpListener::bind(self.config.get_full_addr()).unwrap());

        let _ = self.event_loop().unwrap();
    }
}

impl Default for Plane {
    fn default() -> Self { self::Plane::board() }
}
