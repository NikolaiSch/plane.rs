use {
    anyhow::Result,
    http::{
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
        .route(Route::new(Method::GET, Uri::from_str("/s/e/")?), &(req))?
        .takeoff()
        .await?;

    Ok(())
}

fn req(req: &Request<Vec<String>>) -> Response<Vec<String>> {
    let mut res: Response<Vec<String>> = Response::default();

    res.body_mut().push("Working".to_string());

    res
}
