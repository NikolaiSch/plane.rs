use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("IP could not be successfully parsed from a string")]
    ParseToString {
        #[from]
        source: std::string::ParseError
    },
    #[error("Port {0} is in use")]
    PortInUse(u16)
}

#[derive(Error, Debug)]
pub enum RouteError {
    #[error("")]
    NotFound(Route)
}
