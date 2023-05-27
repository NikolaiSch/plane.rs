#![feature(default_free_fn, assert_matches, try_trait_v2, const_trait_impl)]

use {
    self::body_parser::{
        FromReq,
        ToRes
    },
    http::{
        Request,
        Response
    },
    std::collections::HashMap
};

pub type Req<T> = Request<T>;
pub type Res<T> = Response<T>;

pub type RouteHandler = &'static (dyn Fn(&Req<()>, &mut Res<()>) -> Res<()>);

// pub mod prelude {}

pub mod body_parser;
pub mod error;
pub mod request;
pub mod response;
pub mod route;
pub mod server;
pub mod server_config;
