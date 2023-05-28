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

impl std::fmt::Debug for ServerOpts {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Host(arg0) => f.debug_tuple("Host").field(arg0).finish(),
            Self::Port(arg0) => f.debug_tuple("Port").field(arg0).finish(),
            Self::Fallback(_) => f.debug_tuple("Fallback").finish()
        }
    }
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

    #[instrument(level = Level::DEBUG, fields(ip = self.ip_addr.to_string(), port = self.port))]
    pub fn get_socket_addr(&self) -> SocketAddr {
        let x = SocketAddr::new(self.ip_addr, self.port);
        debug!("The current socket address is {}", x.to_string());
        x
    }

    #[instrument]
    pub fn set(&mut self, opt: ServerOpts) -> anyhow::Result<&mut Self> {
        match opt {
            ServerOpts::Host(ip) => self.ip_addr = IpAddr::from_str(ip)?,
            ServerOpts::Port(port) => self.port = port,
            ServerOpts::Fallback(backup) => {
                self.fallback = Some(backup);
            }
        }
        trace!(opt = ?opt);

        Ok(self)
    }

    #[instrument]
    pub fn parse_ip(ip: &str) -> anyhow::Result<IpAddr> {
        let x = IpAddr::V4(Ipv4Addr::from_str(ip)?);

        debug!(x = ?x);

        Ok(x)
    }
}

impl Default for ServerConfig {
    fn default() -> Self {
        ServerConfig::new()
    }
}
