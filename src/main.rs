use {
    anyhow::Result,
    http::{
        Response,
        StatusCode
    },
    plane_rs::prelude::*
};

fn main() -> Result<()> {
    let mut plane = Plane::board();

    plane
        .set(Host("127.0.0.1"))?
        .set(Port(7574))?
        .route(Method::GET, "/", &|req| {
            let (mut parts, _) = Response::new(()).into_parts();

            parts.version = req.version();
            parts.status = StatusCode::OK;

            return Response::from_parts(parts, vec![]);
        })?
        .takeoff();

    Ok(())
}
