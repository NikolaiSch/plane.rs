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
<<<<<<< HEAD
    std::collections::HashMap,
=======
    std::{
        collections::HashMap,
        str::FromStr
    },
<<<<<<< HEAD
>>>>>>> parent of 8fae1cc (i dont even know at this point)
    tracing::instrument
=======
    tracing::{
        debug,
        instrument,
        trace,
        Level
    }
>>>>>>> parent of 323dabb (Revert "i dont even know at this point")
};

pub type RouteMap = HashMap<Route, RouteHandler>;

#[derive(Hash, PartialEq, Eq, Debug, Clone)]
<<<<<<< HEAD
pub enum Route {
    Standard { path: Uri, method: Method },
    Fallback
=======
pub struct Route {
<<<<<<< HEAD
    pub(crate) path: Uri,
    method:          Method
>>>>>>> parent of 8fae1cc (i dont even know at this point)
=======
    pub path:   Uri,
    pub method: Method
>>>>>>> parent of 323dabb (Revert "i dont even know at this point")
}

impl Route {
    pub fn new(method: Method, path: Uri) -> Route {
        Route::Standard { path, method }
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
    fn get_handler(&self, route: K, res: &Res) -> anyhow::Result<RouteHandler>;

    fn execute_handler(&self, req: &Req) -> anyhow::Result<Res>;
}
<<<<<<< HEAD

impl Handle<Route, RouteHandler> for RouteMap {
    #[instrument(level = "INFO", skip(self))]
    fn get_handler(&self, route: Route) -> anyhow::Result<RouteHandler> {
        if let Some(&handler) = self.get(&route) {
            return Ok(handler);
<<<<<<< HEAD
        } else if let Some(&handler) = self.get(&Route::Fallback) {
=======
        } else if let Some(&handler) = self.get(&Route::new(Method::GET, Uri::from_str("/")?)) {
>>>>>>> parent of 8fae1cc (i dont even know at this point)
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
=======
>>>>>>> parent of 323dabb (Revert "i dont even know at this point")
