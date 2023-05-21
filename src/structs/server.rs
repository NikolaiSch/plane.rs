use std::{
    collections::HashMap,
    net::TcpListener
};

use super::{
    config::ServerConfig,
    route::{
        HashRoute,
        Route
    }
};

pub struct Plane {
    pub config:   ServerConfig,
    pub handlers: HashMap<HashRoute, Route>,

    pub tcp: Option<TcpListener>
}
