use {
    crate::route::Route,
    thiserror::Error
};

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
    #[error("Route not found")]
    NotFound(Route)
}

#[derive(Error, Debug)]
pub enum RequestError {
    #[error("Malformed Request: Empty Request")]
    EmptyRequest,
    #[error("Malformed Request: First Line")]
    FirstLine
}
