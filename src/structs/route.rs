use std::{
    collections::HashMap,
    hash::Hash
};

use crate::{
    enums::request_opts::Method,
    structs::{
        request::Request,
        response::Response
    }
};
pub type RouteHandler = &'static (dyn Fn(&Request) -> Response);
pub type RouteMap = HashMap<Route, RouteHandler>;

#[derive(Hash, PartialEq, PartialOrd, Ord, Eq)]
pub enum Route {
    Standard { path: String, method: Method },
    Fallback
}
