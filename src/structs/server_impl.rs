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
        HashRoute,
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

    pub fn set(
        &mut self,
        opt: ServerOpts
    ) -> Result<&mut Plane> {
        match opt {
            ServerOpts::Host(ip) => {
                self.config.ip_addr = ip.parse::<IPType>()?;
                Ok(self)
            }
            ServerOpts::Port(port) => {
                self.config.port = port;
                Ok(self)
            }
            ServerOpts::Subdirectory(path) => {
                self.config.subdirectory = path;
                Ok(self)
            }
        }
    }

    pub fn route(
        &mut self,
        method: Method,
        path: &'static str,
        route: RouteHandler
    ) -> Result<&mut Plane> {
        let hash_route = HashRoute::new(method, path.to_string());
        let route = hash_route.to_route(route);

        self.handlers.insert(hash_route, route);

        return Ok(self);
    }

    fn event_loop(&self) -> Result<()> {
        if let Some(x) = &self.tcp {
            loop {
                let (mut stream, client_addr) = match x.accept() {
                    Ok(x) => x,
                    Err(e) => {
                        return Err(e.into());
                    }
                };
                // let data = &mut String::new();
                // stream.read_to_string(data).unwrap();

                let mut parser =
                    tcp_parser::Parser::new(stream.try_clone().unwrap());
                parser.parse_stream().unwrap();

                let hr = HashRoute::new(
                    parser.data.method.clone(),
                    parser.data.route.clone()
                );

                let route = self.handlers.get(&hr).unwrap();

                let res = (route.handler)(&parser.data);
                println!("returning response");

                stream
                    .write(
                        Response::default()
                            .to_http_string()
                            .as_bytes()
                    )
                    .unwrap();
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
    fn default() -> Self {
        self::Plane::board()
    }
}
