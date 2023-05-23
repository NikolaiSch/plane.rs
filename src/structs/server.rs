use std::{
    collections::HashMap,
    net::TcpListener
};

use super::{
    config::ServerConfig,
    route::{
        Route,
        Route
    }
};

pub struct Plane {
    pub config:   ServerConfig,
    pub handlers: ,

    pub tcp: Option<TcpListener>
}
