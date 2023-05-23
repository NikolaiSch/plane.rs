use std::str::FromStr;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Locale {
    language: String,
    country:  String
}

impl FromStr for Locale {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut sp_s = s.split("-");

        return Ok(Self {
            language: sp_s.next().unwrap().to_string(),
            country:  sp_s.next().unwrap().to_string()
        });
    }
}
