use crate::structs::route::Route;

pub enum ServerOpts {
    Host(&'static str),
    Port(u16),
    Fallback(Option<Route>)
}
