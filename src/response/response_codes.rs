use super::response::{
    self,
    Status
};

pub enum StatusCodes {
    OK,
    NotFound
}

impl StatusCodes {
    pub fn get(self) -> Status {
        match self {
            Self::OK => {
                Status::new("HTTP/1.1".to_string(), 200, "OK".to_string())
            }
            Self::NotFound => {
                Status::new(
                    "HTTP/1.1".to_string(),
                    404,
                    "Not Found".to_string()
                )
            }
        }
    }
}
