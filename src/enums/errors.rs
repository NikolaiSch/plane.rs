use thiserror::Error;

#[derive(Error, Debug)]
pub enum IPAddrError {
    #[error("could not parse IP from string")]
    ParseFromString
}

#[derive(Error, Debug)]
pub enum ServerError {
    #[error("port {0} is already in use")]
    PortInUse(u16)
}

#[derive(Error, Debug)]
pub enum RouteError {
    #[error("doesn't match this path")]
    DoesNotMatchPath,

    #[error("doesn't match this method")]
    DoesNotMatchMethod
}
