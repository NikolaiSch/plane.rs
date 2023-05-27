mod errors;
mod server_config;

#[cfg(test)]
mod server_config_test;

pub mod config {
    pub use super::server_config::{
        ServerConfig,
        ServerOpts,
        ServerOpts::*
    };
}
