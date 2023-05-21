use std::hash::Hash;

use crate::{
    enums::request_opts::Method,
    structs::{
        request::Request,
        response::Response
    }
};

type RouteHandler = &'static (dyn Fn(&Request) -> Response);

pub(crate) trait Handler {
    fn handle(
        &self,
        req: &Request
    ) -> Response;
    fn match_handler(
        &self,
        req: &Request
    ) -> Result<(), anyhow::Error>;
}

pub trait HandlerPub {
    fn handle_if_match(
        &self,
        req: &Request
    ) -> Option<Response>;
}

#[derive(Hash)]
pub struct HashRoute {
    pub path:   &'static str,
    pub method: Method
}

pub struct Route {
    pub(crate) path:   &'static str,
    pub(crate) method: Method,

    pub(crate) handler: RouteHandler
}
