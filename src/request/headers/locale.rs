use std::{
    char,
    str::FromStr
};

use super::errors::HeaderErrors;

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Locale {
    pub language: String,
    pub country:  String
}

impl Locale {
    pub fn new(country: &str, language: &str) -> Locale {
        return Locale {
            country:  country.to_string(),
            language: language.to_string()
        };
    }
}

impl FromStr for Locale {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 5 {
            return Err(HeaderErrors::LocaleError(
                "Malformed Input: Expected exactly 5 characters".to_string()
            )
            .into());
        }

        let mut s_array: [char; 5] = Default::default();
        s.char_indices().for_each(|(i, x)| s_array[i] = x);

        let (country, language) = match s_array {
            [a, b, '-', c, d] => {
                if [a, b, c, d].iter().any(|x| !x.is_alphabetic()) {
                    return Err(HeaderErrors::LocaleError(
                        "Malformed Input: Expected only alphabetic characters"
                            .to_string()
                    )
                    .into());
                }
                (format!("{a}{b}"), format!("{c}{d}"))
            }
            _ => {
                return Err(HeaderErrors::LocaleError(
                    "Malformed Input: Expected format 'xx-xx'".to_string()
                )
                .into());
            }
        };

        return Ok(Self { language, country });
    }
}
