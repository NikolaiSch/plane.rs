use std::net::IpAddr;
pub enum ServerOpts {
    Host(IpAddr),
    Port(u16),
    Fallback(Route)
}
pub struct ServerConfig {
    pub ip_addr:  Option<IpAddr>,
    pub port:     Option<u16>,
    pub fallback: Option<Route>
}

impl ServerConfig {
    pub fn new() -> Self {
        return Self {
            ip_addr:  None,
            port:     None,
            fallback: None
        };
    }

    pub fn get_full_addr(&self) -> anyhow::Result<String> {
        let ip = self.ip_addr;
        let port = self.port;
        Ok(format!("{}:{}", ip, port))
    }

    pub fn set(&mut self, opt: ServerOpts) -> &mut Self {
        match opt {
            ServerOpts::Host(ip) => self.ip_addr = Some(IpAddr::V4(ip));
                        ServerOpts::Port(port) => self.port = Some(ServerOpts::Port(port));
                                    ServerOpts::Fallback(ip) => self.ip_addr = Some(IpAddr::V4(ip));

                                
        }
        self
    }
}
