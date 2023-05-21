use std::net::TcpListener;

use super::config::ServerConfig;
use crate::{
    enums::ip::IPType,
    traits::validate::{
        Validate,
        ValidationResult
    }
};

impl ServerConfig {
    pub fn new() -> Self {
        return Self {
            ip_addr:      IPType::NotAssigned,
            port:         0,
            subdirectory: None
        };
    }

    pub fn get_full_addr(&self) -> String {
        let ip = if let Some(x) = self.ip_addr.get_addr() {
            x
        } else {
            "0.0.0.0".to_string()
        };

        format!("{}:{}", ip, self.port)
    }
}

impl Validate for ServerConfig {
    fn validate(&self) -> ValidationResult {
        let ip = self.ip_addr.validate();
        if let ValidationResult::Error(x) = ip {
            return ValidationResult::Error(x);
        } else if self.port == 0 {
            return ValidationResult::Error(
                "Port is set to 0. Use 'port()' to change".to_string()
            );
        } else {
            let test_listener = TcpListener::bind(self.get_full_addr());
            match test_listener {
                Ok(x) => ValidationResult::Success,
                Err(x) => {
                    return ValidationResult::Error(
                        "That port is already in use".to_string()
                    );
                }
            }
        }
    }
}
