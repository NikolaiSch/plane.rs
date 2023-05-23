use super::route::Route;
use crate::enums::ip::IPType;

pub struct ServerConfig {
    pub ip_addr:  IPType,
    pub port:     u16,
    pub fallback: Option<Route>
}
