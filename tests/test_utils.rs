use serde::de::DeserializeOwned;
use std::io::prelude::*;
use std::io::Write;
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::{fs::File, thread};

pub fn test_listener_with_response(mut buf: Vec<u8>) -> SocketAddr {
    buf.push(0x4);
    // Assigning port 0 requests the OS to assign a free port
    let listener = TcpListener::bind("127.0.0.1:0").expect("Starting test listener failed");
    let addr = listener
        .local_addr()
        .expect("Could not get test listener addr");
    thread::spawn(move || {
        let (mut stream, _addr) = listener.accept().unwrap();
        stream
            .write(&buf)
            .expect("Could not write in test listener");
        stream.flush().expect("Could not flush in test listener");
    });
    addr
}

pub fn load_json_from_file<T>(path: &str) -> T
where
    T: DeserializeOwned,
{
    serde_json::from_reader(File::open(path).expect("Could not open json file"))
        .expect("Could not read json from file")
}

pub fn load_bytes_from_file(path: &str) -> Vec<u8> {
    let mut buf = Vec::with_capacity(0x100);
    File::open(path)
        .expect("Could not open file")
        .read_to_end(&mut buf)
        .expect("Could not read from file");
    buf
}

#[test]
fn connects() {
    let addr = test_listener_with_response(Vec::new());
    TcpStream::connect(addr).expect("Could not create test stream");
}
