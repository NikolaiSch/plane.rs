use std::{
    collections::HashMap,
    error::Error,
    hash::Hash
};

use anyhow::Result;

use super::{
    request::Request,
    response::Response,
    route::{
        Route,
        RouteHandler,
        RouteMap
    }
};
use crate::enums::{
    errors::RouteError,
    request_opts::Method
};

impl Route {
    pub fn new(method: Method, path: String) -> Route {
        return Route::Standard { path, method };
    }
}

pub trait Handle<K, V> {
    fn get_handler(&self, route: K) -> Option<V>;

    fn execute_handler(&self, req: &Request) -> Result<Response, ()>;
}

impl Handle<Route, RouteHandler> for RouteMap {
    fn get_handler(&self, route: Route) -> Option<RouteHandler> {
        if let Some(&handler) = self.get(&route) {
            return Some(handler);
        } else if let Some(&handler) = self.get(&Route::Fallback) {
            return Some(handler);
        }
        None
    }

    // Result<Response>
    fn execute_handler(&self, req: &Request) -> Result<Response, ()> {
        let route = Route::from_request(&req);
        let handler_opt = self.get_handler(route);

        if let Some(handler) = handler_opt {
            return Ok(handler(req));
        }
        Err(())
    }
}

impl Route {
    fn from_request(req: &Request) -> Self {
        Route::Standard {
            path:   req.route.clone(),
            method: req.method.clone()
        }
    }
}
