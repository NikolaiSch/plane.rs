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
        Route { method, path }
    }
}

impl From<&Req> for Route {
    fn from(value: &Req) -> Self {
        let path = value.uri().clone();
        let method = value.method().clone();

        Route::new(method, path)
    }
}

pub trait Handle<K, V> {
    fn get_handler(&self, route: K, res: &Res) -> anyhow::Result<RouteHandler>;

    fn execute_handler(&self, req: &Req) -> anyhow::Result<Res>;
}
