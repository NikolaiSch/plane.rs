use anyhow::Result;
use plane_rs::prelude::*;



fn main() -> Result<()> {
    let mut plane = Plane::board();

    let status = Status {
        code: 200,
        
    }

    plane
        .set(Host("127.0.0.1"))?
        .set(Port(7574))?
        .route(GET, "/", &|&req| {
            , content: () }
        })?
        .takeoff();

    Ok(())
}
