use crate::test_utils::*;
use std::net::TcpStream;
use vndb_rs::sync::client::Client;

#[test]
fn login() {
    let addr = test_listener_with_response(b"ok".to_vec());
    let mut client = Client::new(TcpStream::connect(addr).expect("Could not create test stream"));
    assert!(client.login().is_ok());
}

#[test]
fn login_with_credentials() {
    let addr = test_listener_with_response(b"ok".to_vec());
    let mut client = Client::new(TcpStream::connect(addr).expect("Could not create test stream"));
    assert!(client
        .login_with_credentials("username", "password")
        .is_ok());
}
