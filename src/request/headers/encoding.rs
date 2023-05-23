use std::str::FromStr;

#[derive(Debug)]
pub enum Encoding {
    Unassigned,

    Gzip,
    Compress,
    Deflate,
    Br,
    Identity,
    Asterix
}

impl FromStr for Encoding {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "gzip" => Ok(Self::Gzip),
            "compress" => Ok(Self::Compress),
            "deflate" => Ok(Self::Deflate),
            "br" => Ok(Self::Br),
            "identity" => Ok(Self::Identity),
            "*" => Ok(Self::Asterix),
            _ => Err(())
        }
    }
}
