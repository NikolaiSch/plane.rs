#![feature(
    default_free_fn,
    assert_matches,
    try_trait_v2,
    const_trait_impl,
    associated_type_bounds,
    associated_type_defaults,
    iterator_try_collect,
    async_closure,
    async_fn_in_trait,
    format_args_nl
)]

use {
    self::server::D,
    http::{
        Request,
        Response
    }
};

pub type Req = Request<Vec<String>>;
pub type Res = Response<Vec<String>>;

enum DE {
    One(RouteHandler)
}

pub type RouteHandler = &'static (dyn Fn(&Req) -> Res);

pub mod body_parser;
pub mod error;
pub mod init;
pub mod request;
pub mod route;
pub mod server;
pub mod server_config;

pub mod prelude {
    pub use {
        super::{
            server::Plane,
            server_config::ServerOpts::*
        },
        http::Method
    };
}
