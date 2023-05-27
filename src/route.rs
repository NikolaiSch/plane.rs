use {
    crate::{
        error::RouteError,
        RouteHandler
    },
    http::{
        Method,
        Request
    },
    std::collections::HashMap
};

pub type RouteMap = HashMap<Route, RouteHandler>;

#[derive(Hash, PartialEq, Eq, Debug, Clone)]
pub enum Route {
    Standard { path: String, method: Method },
    Fallback
}

impl Route {
    pub fn new(method: Method, path: String) -> Route {
        return Route::Standard { path, method };
    }

    fn from_request(req: &Request) -> Self {
        Route::Standard {
            path:   req.route.clone(),
            method: req.method.clone()
        }
    }
}

pub trait Handle<K, V> {
    fn get_handler(&self, route: K) -> anyhow::Result<RouteHandler>;

    fn execute_handler(&self, req: &Request) -> anyhow::Result<Response>;
}

impl Handle<Route, RouteHandler> for RouteMap {
    fn get_handler(&self, route: Route) -> anyhow::Result<RouteHandler> {
        if let Some(&handler) = self.get(&route) {
            return Ok(handler);
        } else if let Some(&handler) = self.get(&Route::Fallback) {
            return Ok(handler);
        }
        Err(RouteError::NotFound(route).into())
    }

    fn execute_handler(&self, req: &Request) -> anyhow::Result<Response> {
        let route = Route::from_request(&req);
        let handler = self.get_handler(route)?;

        let res = handler(req);

        Ok(res)
    }
}
