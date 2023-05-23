use std::{
    collections::HashMap,
    net::TcpListener
};

use super::{
    config::ServerConfig,
    route::{
        Route,
        RouteMap
    }
};

pub struct Plane {
    pub config:   ServerConfig,
    pub handlers: RouteMap,

    pub tcp: Option<TcpListener>
}
