use {
    crate::{
        error::RouteError,
        Req,
        Res,
        RouteHandler
    },
    http::{
        Method,
        Uri
    },
    std::collections::HashMap
};

pub type RouteMap = HashMap<Route, RouteHandler>;

#[derive(Hash, PartialEq, Eq, Debug, Clone)]
pub enum Route {
    Standard { path: Uri, method: Method },
    Fallback
}

impl Route {
    pub fn new(method: Method, path: Uri) -> Route {
        return Route::Standard { path, method };
    }
}

impl From<&Req> for Route {
    fn from(value: &Req) -> Self {
        let path = value.uri().clone();
        let method = value.method().clone();

        Route::Standard { path, method }
    }
}

pub trait Handle<K, V> {
    fn get_handler(&self, route: K) -> anyhow::Result<RouteHandler>;

    fn execute_handler(&self, req: &Req) -> anyhow::Result<Res>;
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

    fn execute_handler(&self, req: &Req) -> anyhow::Result<Res> {
        let route = Route::from(req);
        let handler = self.get_handler(route)?;

        let res = handler(req);

        Ok(res)
    }
}
