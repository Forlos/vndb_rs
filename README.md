
# Table of Contents

1.  [Vndb<sub>rs</sub>](#orgf9bedb8)
    1.  [Usage](#org8e4ed77)
    2.  [Features](#orge0c820c)
    3.  [Examples](#org48e02db)
        1.  [Login and get db stats](#orged59cc4)
        2.  [Login and get all info about vn with id 1](#org9c4f509)
        3.  [Login using credentials and set vote to 8.5 for VN with id 17](#org686836c)
    4.  [References](#org0bd5978)


<a id="orgf9bedb8"></a>

# Vndb<sub>rs</sub>

Bare-bones VNDB api

It is recommended to write higher level wrapper around this library rather than using it directly.


<a id="org8e4ed77"></a>

## Usage

Requires Rust 1.42

    [dependencies]
    vndb_rs = "0.1"


<a id="orge0c820c"></a>

## Features

-   [X] All api actions
-   [ ] TLS support
-   [ ] Async


<a id="org48e02db"></a>

## Examples

For now creating TCP stream is on user side. This is subject to change.


<a id="orged59cc4"></a>

### Login and get db stats

    use std::net::TcpStream;
    use vndb_rs::{
        API_URL,
        sync::client::Client,
    };
    
    let mut client = Client::new(TcpStream::connect(API_URL).unwrap());
    let response = client.login();
    println!("{:#?}", response);
    let response = client.get_dbstats();
    println!("{:#?}", response);


<a id="org9c4f509"></a>

### Login and get all info about vn with id 1

    use std::net::TcpStream;
    use vndb_rs::{
        API_URL,
        sync::client::Client,
        get::vn::VN_FLAGS,
    };
    
    let mut client = Client::new(TcpStream::connect(API_URL).unwrap());
    let response = client.login();
    println!("{:#?}", response);
    let response = client.get_vn(&VN_FLAGS, "(id=6540)".to_owned(), None);
    println!("{:#?}", response);


<a id="org686836c"></a>

### Login using credentials and set vote to 8.5 for VN with id 17

    use std::net::TcpStream;
    use vndb_rs::{
        API_URL,
        sync::client::Client,
        common::set::ulist::UListFields,
    };
    
    let mut client = Client::new(TcpStream::connect(API_URL).unwrap());
    let response = client.login_with_credentials("username", "password");
    println!("{:#?}", response);
    let response = client.set_ulist(
        "17".to_owned(),
        UListFields::new(None, None, None, Some(85), None),
    );
    println!("{:#?}", response);


<a id="org0bd5978"></a>

## References

-   <https://vndb.org/d11>

