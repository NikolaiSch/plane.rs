use {
    anyhow::Result,
    http::{
        request::Builder,
        Request
    }
};

pub struct IncomingRequest {
    req: Request<()>
}

impl IncomingRequest {
    fn new() -> Builder {
        Request::builder()
    }
}
