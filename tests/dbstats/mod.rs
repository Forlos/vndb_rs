use crate::test_utils::*;
use std::net::TcpStream;
use vndb_rs::sync::client::Client;

#[test]
fn get_dbstats() {
    let expected = br###"dbstats {
                          "releases":68258,
                          "producers":9896,
                          "users":0,
                          "tags":2558,
                          "posts":0,
                          "chars":89097,
                          "vn":27300,
                          "traits":2743,
                          "threads":0,
                          "staff":20180
                        }"###;
    let addr = test_listener_with_response(expected.to_vec());
    let mut client = Client::new(TcpStream::connect(addr).expect("Could not create test stream"));
    let response = client.get_dbstats().expect("Getting dbstats failed");

    assert_eq!(response.releases, 68258);
    assert_eq!(response.producers, 9896);
    assert_eq!(response.tags, 2558);
    assert_eq!(response.chars, 89097);
    assert_eq!(response.vn, 27300);
    assert_eq!(response.traits, 2743);
    assert_eq!(response.staff, 20180);
}
