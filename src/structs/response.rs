use crate::enums::request_opts::{
    Headers,
    Method
};

pub struct Response {
    pub data: String
}

impl Default for Response {
    fn default() -> Self {
        return Self {
            data: "test".to_string()
        };
    }
}
