pub use std::{
    net::Ipv4Addr,
    str::FromStr
};

use {
    crate::{
        error::ConfigError,
        RouteHandler
    },
    std::net::TcpListener
};

const DEFAULT_IP: Ipv4Addr = Ipv4Addr::UNSPECIFIED;
const DEFAULT_PORT: u16 = 8000;

pub enum ServerOpts<'a> {
    Host(&'a str),
    Port(u16),
    Fallback(RouteHandler)
}
pub struct ServerConfig {
    pub ip_addr:  Ipv4Addr,
    pub port:     u16,
    pub fallback: Option
}

impl ServerConfig {
    pub fn new() -> Self {
        return Self {
            ip_addr:  DEFAULT_IP,
            port:     DEFAULT_PORT,
            fallback: None
        };
    }

    pub fn get_full_addr(&self) -> String {
        let ip = self.ip_addr;
        let port = self.port;

        format!("{ip}:{port}")
    }

    pub fn set(&mut self, opt: ServerOpts) -> anyhow::Result<&mut Self> {
        match opt {
            ServerOpts::Host(ip) => self.ip_addr = Ipv4Addr::from_str(ip)?,
            ServerOpts::Port(port) => self.port = port,
            ServerOpts::Fallback(backup) => self.fallback = Some(backup)
        }
        Ok(self)
    }

    pub fn validate_port(&self) -> anyhow::Result<()> {
        match TcpListener::bind(self.get_full_addr()) {
            Ok(_) => Ok(()),
            Err(_) => Err(ConfigError::PortInUse(self.port.clone()).into())
        }
    }
}
