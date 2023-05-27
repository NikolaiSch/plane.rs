use {
    super::errors::HeaderErrors,
    std::{
        char,
        str::FromStr
    }
};

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Locale {
    pub language: String,
    pub country:  String
}

impl Locale {
    pub fn new(language: &str, country: &str) -> Locale {
        return Locale {
            country:  country.to_string(),
            language: language.to_string()
        };
    }
}

impl FromStr for Locale {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        if s.len() != 5 {
            return Err(HeaderErrors::LocaleError(
                "Malformed Input: Expected exactly 5 characters".to_string()
            )
            .into());
        }

        let mut s_array: [char; 5] = Default::default();
        s.char_indices().for_each(|(i, x)| s_array[i] = x);

        let (language, country) = match s_array {
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
