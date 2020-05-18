//! # vndb_rs
//!
//! Bare-bones VNDB api
//!
//! # Example
//! ```rust
//!use std::net::TcpStream;
//!use vndb_rs::{
//!   API_URL,
//!   sync::client::Client,
//!};
//!
//!let mut client = Client::new(TcpStream::connect(API_URL).unwrap());
//!let response = client.login();
//!println!("{:#?}", response);
//!let response = client.get_dbstats();
//!println!("{:#?}", response);
//! ```

#![warn(rust_2018_idioms, missing_debug_implementations)]

pub mod common;
pub mod sync;

pub(crate) const END_OF_TRANSMISSION: u8 = 0x4;
pub(crate) const SPACE_CHAR: u8 = 0x20;

pub const API_URL: &str = "api.vndb.org:19534";
pub const API_URL_SSL: &str = "api.vndb.org:19535";
