use std::hash::Hash;

use super::{
    request::Request,
    response::Response,
    route::{
        Handler,
        HandlerPub,
        Route
    }
};
use crate::enums::errors::RouteError;

impl Handler for Route {
    fn handle(
        &self,
        req: &Request
    ) -> Response {
        let handler = self.handler.clone();
        handler(req)
    }

    fn match_handler(
        &self,
        req: &Request
    ) -> Result<(), anyhow::Error> {
        let path: bool = (req.path)
            .clone()
            .is_some_and(|p: String| return p == self.path);

        if path {
            let method = (req.method).clone().unwrap() == self.method;
            match method {
                true => return Ok(()),
                false => {
                    return Err(RouteError::DoesNotMatchMethod.into());
                }
            }
        } else {
            return Err(RouteError::DoesNotMatchPath.into());
        }
    }
}

impl HandlerPub for Route {
    fn handle_if_match(
        &self,
        req: &Request
    ) -> Option<Response> {
        if let Ok(_) = self.match_handler(req) {
            let data = self.handle(req);
            return Some(data);
        } else {
            return None;
        }
    }
}
impl Hash for Route {
    fn hash<H: std::hash::Hasher>(
        &self,
        state: &mut H
    ) {
        self.path.hash(state);
        self.method.hash(state);
    }
}
