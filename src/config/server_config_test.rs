use std::net::TcpListener;

use super::server_config::{
    self,
    *
};
use crate::{
    response::response::Response,
    routing::route::RouteHandler
};

const ERR_IP_MESSAGE: &str = "IP address is incorrect";

#[test]
fn initialise_new_server_config() {
    let server_config = ServerConfig::new();

    assert_eq!(
        server_config.ip_addr,
        Ipv4Addr::new(0, 0, 0, 0),
        "{ERR_IP_MESSAGE}"
    );
    assert_eq!(server_config.port, 8000);
    assert!(server_config.fallback.is_none());

    assert_eq!(server_config.get_full_addr(), "0.0.0.0:8000")
}

#[test]
fn set_ip_addr() {
    let mut server_config = ServerConfig::new();

    server_config.set(ServerOpts::Host("127.0.0.1")).unwrap();

    assert_eq!(
        server_config.ip_addr,
        Ipv4Addr::new(127, 0, 0, 1),
        "{ERR_IP_MESSAGE}"
    );
    assert_eq!(server_config.port, 8000);
    assert!(server_config.fallback.is_none());

    assert_eq!(server_config.get_full_addr(), "127.0.0.1:8000")
}

#[test]
fn set_port() {
    let mut server_config = ServerConfig::new();

    server_config.set(ServerOpts::Port(3000)).unwrap();

    assert_eq!(
        server_config.ip_addr,
        Ipv4Addr::new(0, 0, 0, 0),
        "{ERR_IP_MESSAGE}"
    );
    assert_eq!(server_config.port, 3000);
    assert!(server_config.fallback.is_none());

    assert_eq!(server_config.get_full_addr(), "0.0.0.0:3000")
}

#[test]
fn set_fallback() {
    let mut server_config = ServerConfig::new();

    let route: RouteHandler = &|_req| Response::default();

    server_config.set(ServerOpts::Fallback(route)).unwrap();

    assert_eq!(
        server_config.ip_addr,
        Ipv4Addr::new(0, 0, 0, 0),
        "{ERR_IP_MESSAGE}"
    );
    assert_eq!(server_config.port, 8000);
    assert!(server_config.fallback.is_some());

    assert_eq!(server_config.get_full_addr(), "0.0.0.0:8000")
}

#[test]
fn port_in_use_false() {
    let server_config = ServerConfig::new();

    let is_free = server_config.validate_port();

    assert!(is_free.is_ok(), "Port {} is in use", server_config.port);
}

#[test]
#[ignore = "can mess up tests when multithreaded"]
fn port_in_use_true() {
    let server_config = ServerConfig::new();

    let _listener = TcpListener::bind(server_config.get_full_addr()).unwrap();

    let is_free = server_config.validate_port();

    assert!(is_free.is_err());
}
