use anyhow::Result;

use super::route::{
    self,
    Route,
    RouteMap
};
use crate::{
    request::{
        headers::{
            http_version::HTTPVersion,
            method::MimeType
        },
        request::Request
    },
    response::response::{
        Content,
        Response,
        Status
    },
    routing::{
        errors::RouteError,
        route::Handle
    }
};

const M: MimeType = MimeType::GET;
const P: &str = "/";

fn create_route_map() -> RouteMap { RouteMap::new() }

#[test]
fn route_integration() {
    let mut map = create_route_map();

    let route = Route::new(M, P.to_string());

    let insert = map.insert(route, &|req| {
        Response {
            status:  Status::default(),
            content: Content::new("text/plain", &req.route)
        }
    });

    assert!(insert.is_none());
    assert!(map.contains_key(&Route::new(M, P.to_string())));

    let req = get_new_req();

    let res = map.execute_handler(&req).unwrap();

    assert_eq!(res.content.data, P.to_string());
}

fn get_new_req() -> Request {
    let mut req = Request::default();

    req.client.http_version = HTTPVersion::V1_1;

    req.method = M;
    req.route = P.to_string();

    req
}
