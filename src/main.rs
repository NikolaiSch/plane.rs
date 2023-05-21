use anyhow::Result;
use plane_rs::prelude::*;

fn main() -> Result<()> {
    let mut plane = Plane::board();

    plane
        .set(Host("127.0.0.1"))?
        .set(Port(7575))?
        .takeoff();

    Ok(())
}
