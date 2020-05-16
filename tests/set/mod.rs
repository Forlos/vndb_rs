use crate::test_utils::*;
use std::net::TcpStream;
use vndb_rs::{common::set::ulist::UListFields, sync::client::Client};

#[test]
fn set_ulist() {
    let addr = test_listener_with_response(b"ok".to_vec());
    let mut client = Client::new(TcpStream::connect(addr).expect("Could not create test stream"));
    assert!(client
        .set_ulist(1, UListFields::new(None, None, None, None, None))
        .is_ok());
}

#[test]
fn delete_ulist() {
    let addr = test_listener_with_response(b"ok".to_vec());
    let mut client = Client::new(TcpStream::connect(addr).expect("Could not create test stream"));
    assert!(client.delete_ulist(1).is_ok());
}
