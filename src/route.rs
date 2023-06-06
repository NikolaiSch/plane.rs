use {
    crate::{
        error::RouteError,
        Req,
        Res,
        RouteHandler
    },
    anyhow::bail,
    http::{
        Method,
        Response,
        Uri
    },
    std::{
        collections::HashMap,
        str::FromStr
    },
    tracing::{
        debug,
        instrument,
        trace,
        Level
    }
};

pub type RouteMap = HashMap<Route, RouteHandler>;

#[derive(Hash, PartialEq, Eq, Debug, Clone)]
pub struct Route {
    pub path:   Uri,
    pub method: Method
}

impl Route {
    pub fn new(method: Method, path: Uri) -> Route {
        Route {
            method, path
        }
    }
}

impl From<&Req> for Route {
    fn from(value: &Req) -> Self {
        let path = value.uri().clone();
        let method = value.method().clone();

        Route {
            method, path
        }
    }
}

pub trait Handle<K, V> {
    fn get_handler(&self, route: K) -> anyhow::Result<RouteHandler>;

    fn execute_handler(&self, req: &Req) -> anyhow::Result<Res>;
}

impl Handle<Route, RouteHandler> for RouteMap {
    #[instrument(level = "INFO", skip(self))]
    fn get_handler(&self, route: Route) -> anyhow::Result<RouteHandler> {
        if let Some(&handler) = self.get(&route) {
            return Ok(handler);
        } else if let Some(&handler) = self.get(&Route::new(Method::GET, Uri::from_str("/")?)) {
            return Ok(handler);
        }
        Err(RouteError::NotFound(route).into())
    }

    fn execute_handler(&self, req: &Req) -> anyhow::Result<Res> {
        let route = Route::from(req);
        let handler = self.get_handler(route)?;

        let res = handler(req);

        Ok(res)
    }
}

