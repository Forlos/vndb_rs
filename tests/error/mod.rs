use crate::test_utils::*;
use std::net::TcpStream;
use vndb_rs::{common::error::VndbError, sync::client::Client};

#[test]
fn parse_error() {
    let mut buf = b"error ".to_vec();
    let expected = VndbError::Parse {
        msg: "Invalid command or argument".to_owned(),
    };
    buf.extend(serde_json::to_vec(&expected).expect("Could not serialize error"));

    let addr = test_listener_with_response(buf);
    let mut client = Client::new(TcpStream::connect(addr).expect("Could not create test stream"));
    let error = client.login();

    assert!(error.is_err());
    let error = error.err().unwrap();
    assert_eq!(error, expected);
}

#[test]
fn missing_error() {
    let mut buf = b"error ".to_vec();
    let expected = VndbError::Missing {
        msg: "Missing field".to_owned(),
        field: "id".to_owned(),
    };
    buf.extend(serde_json::to_vec(&expected).expect("Could not serialize error"));

    let addr = test_listener_with_response(buf);
    let mut client = Client::new(TcpStream::connect(addr).expect("Could not create test stream"));
    let error = client.login();

    assert!(error.is_err());
    let error = error.err().unwrap();
    assert_eq!(error, expected);
}

#[test]
fn bad_arg_error() {
    let mut buf = b"error ".to_vec();
    let expected = VndbError::Missing {
        msg: "Bad argument".to_owned(),
        field: "id".to_owned(),
    };
    buf.extend(serde_json::to_vec(&expected).expect("Could not serialize error"));

    let addr = test_listener_with_response(buf);
    let mut client = Client::new(TcpStream::connect(addr).expect("Could not create test stream"));
    let error = client.login();

    assert!(error.is_err());
    let error = error.err().unwrap();
    assert_eq!(error, expected);
}

#[test]
fn need_login_error() {
    let mut buf = b"error ".to_vec();
    let expected = VndbError::NeedLogin {
        msg: "Not logged in".to_owned(),
    };
    buf.extend(serde_json::to_vec(&expected).expect("Could not serialize error"));

    let addr = test_listener_with_response(buf);
    let mut client = Client::new(TcpStream::connect(addr).expect("Could not create test stream"));
    let error = client.login();

    assert!(error.is_err());
    let error = error.err().unwrap();
    assert_eq!(error, expected);
}

#[test]
fn throttled_error() {
    let mut buf = b"error ".to_vec();
    let expected = VndbError::Throttled {
        msg: "Not logged in".to_owned(),
        typ: "Message".to_owned(),
        minwait: 10.0,
        fullwait: 100.0,
    };
    buf.extend(serde_json::to_vec(&expected).expect("Could not serialize error"));

    let addr = test_listener_with_response(buf);
    let mut client = Client::new(TcpStream::connect(addr).expect("Could not create test stream"));
    let error = client.login();

    assert!(error.is_err());
    let error = error.err().unwrap();
    assert_eq!(error, expected);
}

#[test]
fn auth_error() {
    let mut buf = b"error ".to_vec();
    let expected = VndbError::Auth {
        msg: "Could not authorize".to_owned(),
    };
    buf.extend(serde_json::to_vec(&expected).expect("Could not serialize error"));

    let addr = test_listener_with_response(buf);
    let mut client = Client::new(TcpStream::connect(addr).expect("Could not create test stream"));
    let error = client.login();

    assert!(error.is_err());
    let error = error.err().unwrap();
    assert_eq!(error, expected);
}

#[test]
fn loggedin_error() {
    let mut buf = b"error ".to_vec();
    let expected = VndbError::LoggedIn {
        msg: "Already logged in".to_owned(),
    };
    buf.extend(serde_json::to_vec(&expected).expect("Could not serialize error"));

    let addr = test_listener_with_response(buf);
    let mut client = Client::new(TcpStream::connect(addr).expect("Could not create test stream"));
    let error = client.login();

    assert!(error.is_err());
    let error = error.err().unwrap();
    assert_eq!(error, expected);
}

#[test]
fn gettype_error() {
    let mut buf = b"error ".to_vec();
    let expected = VndbError::GetType {
        msg: "Unknown get type".to_owned(),
    };
    buf.extend(serde_json::to_vec(&expected).expect("Could not serialize error"));

    let addr = test_listener_with_response(buf);
    let mut client = Client::new(TcpStream::connect(addr).expect("Could not create test stream"));
    let error = client.login();

    assert!(error.is_err());
    let error = error.err().unwrap();
    assert_eq!(error, expected);
}

#[test]
fn getinfo_error() {
    let mut buf = b"error ".to_vec();
    let expected = VndbError::GetInfo {
        msg: "Unknown get flag".to_owned(),
        flag: "Unknown".to_owned(),
    };
    buf.extend(serde_json::to_vec(&expected).expect("Could not serialize error"));

    let addr = test_listener_with_response(buf);
    let mut client = Client::new(TcpStream::connect(addr).expect("Could not create test stream"));
    let error = client.login();

    assert!(error.is_err());
    let error = error.err().unwrap();
    assert_eq!(error, expected);
}

#[test]
fn filter_error() {
    let mut buf = b"error ".to_vec();
    let expected = VndbError::Filter {
        msg: "Invalid filter".to_owned(),
        field: "id".to_owned(),
        op: "=".to_owned(),
        value: "123".to_owned(),
    };
    buf.extend(serde_json::to_vec(&expected).expect("Could not serialize error"));

    let addr = test_listener_with_response(buf);
    let mut client = Client::new(TcpStream::connect(addr).expect("Could not create test stream"));
    let error = client.login();

    assert!(error.is_err());
    let error = error.err().unwrap();
    assert_eq!(error, expected);
}

#[test]
fn settype_error() {
    let mut buf = b"error ".to_vec();
    let expected = VndbError::SetType {
        msg: "Unknown set type".to_owned(),
    };
    buf.extend(serde_json::to_vec(&expected).expect("Could not serialize error"));

    let addr = test_listener_with_response(buf);
    let mut client = Client::new(TcpStream::connect(addr).expect("Could not create test stream"));
    let error = client.login();

    assert!(error.is_err());
    let error = error.err().unwrap();
    assert_eq!(error, expected);
}

#[test]
fn other_error() {
    let mut buf = b"error ".to_vec();
    let expected = VndbError::Other {
        msg: "Unknown error".to_owned(),
    };
    buf.extend(serde_json::to_vec(&expected).expect("Could not serialize error"));

    let addr = test_listener_with_response(buf);
    let mut client = Client::new(TcpStream::connect(addr).expect("Could not create test stream"));
    let error = client.login();

    assert!(error.is_err());
    let error = error.err().unwrap();
    assert_eq!(error, expected);
}
