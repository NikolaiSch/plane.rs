pub use std::{
    net::Ipv4Addr,
    str::FromStr
};

use {
    crate::{
        RouteHandler
    },
    std::{
        fmt::Formatter,
        net::{
            IpAddr,
            SocketAddr
        }
    },
    tracing::*
};

const DEFAULT_IP: Ipv4Addr = Ipv4Addr::UNSPECIFIED;
const DEFAULT_PORT: u16 = 8000;

pub enum ServerOpts {
    Host(&'static str),
    Port(u16),
    Fallback(RouteHandler)
}

pub struct ServerConfig {
    pub ip_addr:  IpAddr,
    pub port:     u16,
    pub fallback: Option<RouteHandler>
}

impl std::fmt::Debug for ServerConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ServerConfig")
            .field("ip_addr", &self.ip_addr)
            .field("port", &self.port)
            .finish()
    }
}

impl ServerConfig {
    pub fn new() -> Self {
        Self {
            ip_addr:  IpAddr::V4(DEFAULT_IP),
            port:     DEFAULT_PORT,
            fallback: None
        }
    }

    #[instrument(level = Level::TRACE, fields(ip = self.ip_addr.to_string(), port = self.port))]
    pub fn get_socket_addr(&self) -> SocketAddr {
        SocketAddr::new(self.ip_addr, self.port)
    }

    pub fn set(&mut self, opt: ServerOpts) -> anyhow::Result<&mut Self> {
        let _span = span!(Level::TRACE, "setting_server_conf_opts").entered();

        match opt {
            ServerOpts::Host(ip) => self.ip_addr = IpAddr::from_str(ip)?,
            ServerOpts::Port(port) => self.port = port,
            ServerOpts::Fallback(backup) => {
                self.fallback = Some(backup);
            }
        }

        Ok(self)
    }

    pub fn parse_ip(ip: &str) -> anyhow::Result<IpAddr> {
        let _span = span!(Level::TRACE, "parsing_ip_address").entered();

        Ok(IpAddr::V4(Ipv4Addr::from_str(ip)?))
    }
}

impl Default for ServerConfig {
    fn default() -> Self {
        ServerConfig::new()
    }
}
