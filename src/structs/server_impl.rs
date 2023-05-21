use std::{
    collections::HashMap,
    io::Read,
    net::TcpListener
};

use anyhow::Result;

use super::{
    config::ServerConfig,
    server::Plane,
    tcp_parser
};
use crate::{
    enums::{
        config::ServerOpts,
        ip::IPType
    },
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

    fn event_loop(&self) -> Result<()> {
        if let Some(x) = &self.tcp {
            loop {
                let (stream, client_addr) = match x.accept() {
                    Ok(x) => x,
                    Err(e) => {
                        return Err(e.into());
                    }
                };
                // let data = &mut String::new();
                // stream.read_to_string(data).unwrap();

                let mut parser = tcp_parser::Parser::new(stream);
                dbg!(1);
                dbg!(parser.parse_stream().unwrap());
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