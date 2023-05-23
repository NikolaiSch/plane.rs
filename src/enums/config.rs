use crate::structs::route::Route;

#[derive(Debug)]
pub enum ServerOpts {
    Host(&'static str),
    Port(u16),
    Fallback(Option<Route>)
}
