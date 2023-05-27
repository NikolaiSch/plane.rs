use thiserror::Error;

use super::route::Route;

#[derive(Error, Debug)]
pub enum RouteError {
    #[error("")]
    NotFound(Route)
}
