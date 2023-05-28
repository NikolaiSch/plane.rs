use {
    anyhow::Result,
    http::{
        Request,
        Response,
        StatusCode
    },
    plane_rs::{
        init::init,
        prelude::*
    },
    tracing::{
        instrument,
        Level
    }
};

#[tokio::main]
#[instrument(level = Level::INFO, name = "main_function")]
async fn main() -> Result<()> {
    init();

    Plane::board()
        .set(Host("127.0.0.1"))?
        .set(Port(7574))?
        .route(Route, &handle(req));

    Ok(())
}
