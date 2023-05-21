#![feature(hasher_prefixfree_extras)]

pub(crate) mod enums;
pub(crate) mod structs;
pub(crate) mod traits;

pub mod prelude {
    pub use crate::{
        enums::{
            config::ServerOpts::*,
            request_opts::Method::*
        },
        structs::{
            response::Response,
            server::Plane
        }
    };
}
