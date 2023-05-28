use {
    anyhow::Result,
    http::{
        Response,
        StatusCode
    },
    plane_rs::{
        init::init,
        prelude::*
    },
    tracing::{
        dispatcher,
        dispatcher::Dispatch,
        info,
        instrument,
        metadata::LevelFilter,
        span,
        Level
    },
    tracing_subscriber::{
        fmt::format::FmtSpan,
        FmtSubscriber
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
