pub use std::{
    net::Ipv4Addr,
    str::FromStr
};

use {
    crate::{
        error::ConfigError,
        RouteHandler
    },
    std::net::{
        IpAddr,
        SocketAddr,
        TcpListener
    }
};

const DEFAULT_IP: Ipv4Addr = Ipv4Addr::UNSPECIFIED;
const DEFAULT_PORT: u16 = 8000;

pub enum ServerOpts<'a> {
    Host(&'a str),
    Port(u16),
    Fallback(RouteHandler)
}
pub struct ServerConfig {
    pub ip_addr:  IpAddr,
    pub port:     u16,
    pub fallback: Option<RouteHandler>
}

impl ServerConfig {
    pub fn new() -> Self {
        Self {
            ip_addr:  IpAddr::V4(DEFAULT_IP),
            port:     DEFAULT_PORT,
            fallback: None
        }
    }

    pub fn get_socket_addr(&self) -> SocketAddr {
        SocketAddr::new(self.ip_addr, self.port)
    }

    pub fn set(&mut self, opt: ServerOpts) -> anyhow::Result<&mut Self> {
        match opt {
            ServerOpts::Host(ip) => self.ip_addr = IpAddr::from_str(ip)?,
            ServerOpts::Port(port) => self.port = port,
            ServerOpts::Fallback(backup) => {
                self.fallback = Some(backup);
            }
        }
        Ok(self)
    }

    pub fn validate_port(&self) -> anyhow::Result<()> {
        match TcpListener::bind(self.get_socket_addr()) {
            Ok(_) => Ok(()),
            Err(_) => Err(ConfigError::PortInUse(self.port).into())
        }
    }

    pub fn parse_ip(ip: &str) -> anyhow::Result<IpAddr> {
        Ok(IpAddr::V4(Ipv4Addr::from_str(ip)?))
    }
}

impl Default for ServerConfig {
    fn default() -> Self {
        ServerConfig::new()
    }
}
