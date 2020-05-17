#![warn(rust_2018_idioms, missing_debug_implementations)]

pub mod common;
pub mod sync;

pub(crate) const END_OF_TRANSMISSION: u8 = 0x4;
pub(crate) const SPACE_CHAR: u8 = 0x20;

pub const API_URL: &str = "api.vndb.org:19534";
pub const API_URL_SSL: &str = "api.vndb.org:19535";
