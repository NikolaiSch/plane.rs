#![feature(
    default_free_fn,
    assert_matches,
    try_trait_v2,
    const_trait_impl,
    associated_type_bounds,
    associated_type_defaults,
    iterator_try_collect,
    async_closure
)]

use http::{
    Request,
    Response
};

pub type Req = Request<Vec<String>>;
pub type Res = Response<Vec<String>>;

pub type RouteHandler = &'static (dyn Fn(&Req) -> Res);

mod body_parser;
mod error;
mod request;
mod route;
mod server;
mod server_config;

pub mod prelude {
    pub use {
        super::{
            server::Plane,
            server_config::ServerOpts::*
        },
        http::Method
    };
}
