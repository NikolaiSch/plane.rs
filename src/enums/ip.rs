use std::str::FromStr;

use super::errors::IPAddrError;
use crate::traits::validate::{
    Validate,
    ValidationResult
};

pub enum IPType {
    NotAssigned,

    Address([u8; 4]),
    Host(String)
}

impl IPType {
    pub fn get_addr(&self) -> Option<String> {
        match self {
            IPType::NotAssigned => None,

            IPType::Address(x) => {
                let out_string =
                    x.to_owned().map(|ip_part| ip_part.to_string()).join(".");
                Some(out_string)
            }

            IPType::Host(x) => Some(x.to_owned())
        }
    }
}

impl FromStr for IPType {
    type Err = IPAddrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split_str = s.clone().split(".");

        if split_str
            .clone()
            .filter(|&x| {
                match x.parse::<u8>() {
                    Ok(_) => return true,
                    Err(_) => return false
                }
            })
            .count()
            == 4
        {
            let mut temp_array = [0 as u8; 4];
            for (i, val) in split_str.enumerate() {
                let parsed = val.parse::<u8>().unwrap();
                temp_array[i] = parsed;
            }
            return Ok(IPType::Address(temp_array));
        } else if s.contains("localhost") {
            return Ok(IPType::Host("localhost".to_string()));
        } else {
            return Err(IPAddrError::ParseFromString.into());
        }
    }
}

impl Validate for IPType {
    fn validate(&self) -> ValidationResult {
        match &self {
            Self::NotAssigned => {
                return ValidationResult::Error(
                    "Unassigned IP address use '.host()'".to_string()
                );
            }
            Self::Address(x) => {
                return ValidationResult::Success;
            }
            Self::Host(x) => {
                if x == &"localhost".to_string() {
                    return ValidationResult::Success;
                } else {
                    return ValidationResult::Error(
                        "only able to use localhost at this moment".to_string()
                    );
                }
            }
        }
    }
}
