use {
    anyhow::Result,
    http::{
        Request,
        Response,
        StatusCode,
        Uri
    },
    plane_rs::{
        init::init,
        prelude::*,
        route::Route
    },
    std::str::FromStr,
    tracing::{
        instrument,
        Level
    }
};

#[tokio::main]
#[instrument(level = Level::INFO, name = "main_span")]
async fn main() -> Result<()> {
    init();

    Plane::board()
        .set(Host("127.0.0.1"))?
        .set(Port(7574))?
        .route(Method::GET, "/", &|req| {
            let (mut parts, _) = Response::new(()).into_parts();

            parts.version = req.version();
            parts.status = StatusCode::OK;

            Response::from_parts(parts, vec![])
        })?
        .takeoff()
        .await?;

    Ok(())
}

fn req(req: &Request<Vec<String>>) -> Response<Vec<String>> {
    let mut res: Response<Vec<String>> = Response::default();

    res.body_mut().push("Working".to_string());

    res
}
