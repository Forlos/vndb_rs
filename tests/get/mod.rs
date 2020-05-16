use crate::test_utils::*;
use std::net::TcpStream;
use vndb_rs::{
    common::get::{
        character::{GetCharacterResults, CHARACTER_FLAGS},
        producer::{GetProducerResults, PRODUCER_FLAGS},
        release::{GetReleaseResults, RELEASE_FLAGS},
        staff::{GetStaffResults, STAFF_FLAGS},
        ulist::{GetUListResults, ULIST_FLAGS},
        ulistlabels::{GetUListLabelsResults, ULIST_LABELS_FLAGS},
        user::{GetUserResults, USER_FLAGS},
        vn::{GetVnResults, VN_FLAGS},
    },
    sync::client::Client,
};

#[test]
fn get_vn() {
    const DATA_PATH: &str = "tests/data/vn.json";

    let mut buf = b"results ".to_vec();
    let json: GetVnResults = load_json_from_file(DATA_PATH);
    buf.extend(load_bytes_from_file(DATA_PATH));

    let addr = test_listener_with_response(buf);
    let mut client = Client::new(TcpStream::connect(addr).expect("Could not create test stream"));

    let response = client
        .get_vn(&VN_FLAGS, "(id = 1)".to_owned(), None)
        .expect("Getting vns failed");

    assert_eq!(json.results, response.results);
    assert_eq!(
        json.items.iter().next().expect("Could not parse json"),
        response.items.iter().next().expect("Expected 1 item")
    );
}

#[test]
fn get_release() {
    const DATA_PATH: &str = "tests/data/release.json";

    let mut buf = b"results ".to_vec();
    let json: GetReleaseResults = load_json_from_file(DATA_PATH);
    buf.extend(load_bytes_from_file(DATA_PATH));

    let addr = test_listener_with_response(buf);
    let mut client = Client::new(TcpStream::connect(addr).expect("Could not create test stream"));

    let response = client
        .get_release(&RELEASE_FLAGS, "(id > 1000)".to_owned(), None)
        .expect("Getting releases failed");

    assert_eq!(json.results, response.results);
    assert_eq!(
        json.items.iter().next().expect("Could not parse json"),
        response.items.iter().next().expect("Expected 1 item")
    );
}

#[test]
fn get_producer() {
    const DATA_PATH: &str = "tests/data/producer.json";

    let mut buf = b"results ".to_vec();
    let json: GetProducerResults = load_json_from_file(DATA_PATH);
    buf.extend(load_bytes_from_file(DATA_PATH));

    let addr = test_listener_with_response(buf);
    let mut client = Client::new(TcpStream::connect(addr).expect("Could not create test stream"));

    let response = client
        .get_producer(&PRODUCER_FLAGS, "(id > 1000)".to_owned(), None)
        .expect("Getting producer failed");

    assert_eq!(json.results, response.results);
    assert_eq!(
        json.items.iter().next().expect("Could not parse json"),
        response.items.iter().next().expect("Expected 1 item")
    );
}

#[test]
fn get_character() {
    const DATA_PATH: &str = "tests/data/character.json";

    let mut buf = b"results ".to_vec();
    let json: GetCharacterResults = load_json_from_file(DATA_PATH);
    buf.extend(load_bytes_from_file(DATA_PATH));

    let addr = test_listener_with_response(buf);
    let mut client = Client::new(TcpStream::connect(addr).expect("Could not create test stream"));

    let response = client
        .get_character(&CHARACTER_FLAGS, "(id > 1000)".to_owned(), None)
        .expect("Getting character failed");

    assert_eq!(json.results, response.results);
    assert_eq!(
        json.items.iter().next().expect("Could not parse json"),
        response.items.iter().next().expect("Expected 1 item")
    );
}

#[test]
fn get_staff() {
    const DATA_PATH: &str = "tests/data/staff.json";

    let mut buf = b"results ".to_vec();
    let json: GetStaffResults = load_json_from_file(DATA_PATH);
    buf.extend(load_bytes_from_file(DATA_PATH));

    let addr = test_listener_with_response(buf);
    let mut client = Client::new(TcpStream::connect(addr).expect("Could not create test stream"));

    let response = client
        .get_staff(&STAFF_FLAGS, "(id > 1000)".to_owned(), None)
        .expect("Getting staff failed");

    assert_eq!(json.results, response.results);
    assert_eq!(
        json.items.iter().next().expect("Could not parse json"),
        response.items.iter().next().expect("Expected 1 item")
    );
}

#[test]
fn get_user() {
    const DATA_PATH: &str = "tests/data/user.json";

    let mut buf = b"results ".to_vec();
    let json: GetUserResults = load_json_from_file(DATA_PATH);
    buf.extend(load_bytes_from_file(DATA_PATH));

    let addr = test_listener_with_response(buf);
    let mut client = Client::new(TcpStream::connect(addr).expect("Could not create test stream"));

    let response = client
        .get_user(&USER_FLAGS, "(id > 1000)".to_owned(), None)
        .expect("Getting user failed");

    assert_eq!(json.results, response.results);
    assert_eq!(
        json.items.iter().next().expect("Could not parse json"),
        response.items.iter().next().expect("Expected 1 item")
    );
}

#[test]
fn get_ulist_labels() {
    const DATA_PATH: &str = "tests/data/ulistlabels.json";

    let mut buf = b"results ".to_vec();
    let json: GetUListLabelsResults = load_json_from_file(DATA_PATH);
    buf.extend(load_bytes_from_file(DATA_PATH));

    let addr = test_listener_with_response(buf);
    let mut client = Client::new(TcpStream::connect(addr).expect("Could not create test stream"));

    let response = client
        .get_ulist_labels(&ULIST_LABELS_FLAGS, "(id > 1000)".to_owned(), None)
        .expect("Getting ulist labels failed");

    assert_eq!(json.results, response.results);
    assert_eq!(
        json.items.iter().next().expect("Could not parse json"),
        response.items.iter().next().expect("Expected 1 item")
    );
}

#[test]
fn get_ulist() {
    const DATA_PATH: &str = "tests/data/ulist.json";

    let mut buf = b"results ".to_vec();
    let json: GetUListResults = load_json_from_file(DATA_PATH);
    buf.extend(load_bytes_from_file(DATA_PATH));

    let addr = test_listener_with_response(buf);
    let mut client = Client::new(TcpStream::connect(addr).expect("Could not create test stream"));

    let response = client
        .get_ulist(&ULIST_FLAGS, "(id > 1000)".to_owned(), None)
        .expect("Getting ulist failed");

    assert_eq!(json.results, response.results);
    assert_eq!(
        json.items.iter().next().expect("Could not parse json"),
        response.items.iter().next().expect("Expected 1 item")
    );
}
