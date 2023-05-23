// use anyhow::Result;
// use plane_rs::prelude::*;

// fn main() -> Result<()> {
//     let mut plane = Plane::board();

//     plane
//         .set(Host("127.0.0.1"))?
//         .set(Port(7574))?
//         .route(GET, "/", &|_req| {
//             Response {
//                 status:  Status {
//                     code:         200,
//                     http_version: HTTPStatus::V1_1,
//                     message:      format!("OK")
//                 },
//                 content: Content {
//                     mime_type: "text/html".to_string(),
//                     data:      "<h1>this is cool</h1>".to_string()
//                 }
//             }
//         })?
//         .route(GET, "/hello", &|_req| {
//             Response {
//                 status:  Status {
//                     code:         200,
//                     http_version: HTTPStatus::V1_1,
//                     message:      format!("OK")
//                 },
//                 content: Content {
//                     mime_type: "text/html".to_string(),
//                     data:      "<h1>this is cool, on page hello!</h1>"
//                         .to_string()
//                 }
//             }
//         })?
//         .takeoff();

//     Ok(())
// }
