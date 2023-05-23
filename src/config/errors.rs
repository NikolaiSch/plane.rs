use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("The property 'IP' has not been assigned")]
    UnassignedIPAddress,
    #[error("IP could not be successfully parsed from a string")]
    ParseToString {
        #[from]
        source: std::string::ParseError
    },
    #[error("Port is already in use")]
    PortInUse
}
