use super::error::VndbResult;
use crate::{common::error::VndbError, END_OF_TRANSMISSION, SPACE_CHAR};
use serde::Deserialize;

#[derive(Debug)]
pub enum Response {
    Ok,
    Error(VndbError),
    DbStats(DbStatsResponse),
    Results(Vec<u8>),
}

impl Response {
    pub(crate) fn parse_response(buf: &[u8]) -> VndbResult<Self> {
        Ok(match buf {
            [0x6F, 0x6B, END_OF_TRANSMISSION] => Self::Ok,
            [0x65, 0x72, 0x72, 0x6f, 0x72, SPACE_CHAR, response @ .., END_OF_TRANSMISSION] => {
                Self::Error(VndbError::parse_error(response).unwrap())
            }
            [0x64, 0x62, 0x73, 0x74, 0x61, 0x74, 0x73, SPACE_CHAR, response @ .., END_OF_TRANSMISSION] => {
                Self::DbStats(DbStatsResponse::parse(response).unwrap())
            }
            [0x72, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x73, SPACE_CHAR, response @ .., END_OF_TRANSMISSION] => {
                Self::Results(response.to_vec())
            }
            _ => todo!(),
        })
    }
    // pub(crate) fn parse_reponse(buf: &[u8]) -> Self {
    //     let buf = match buf.split_last() {
    //         Some((0x04, buf)) => buf,
    //         _ => {
    //             return Self::Error(VndbError::Other {
    //                 msg: "Invalid response from server".to_owned(),
    //             })
    //         }
    //     };
    //     let response_type = ResponseType::from_buf(&buf);
    //     match response_type {
    //         ResponseType::Ok => Self::Ok,
    //         ResponseType::Error => {
    //             let response = buf.splitn(2, |b| *b == SPACE_CHAR).skip(1).next().unwrap();
    //             Self::Error(VndbError::parse_error(&response).unwrap())
    //         }
    //         ResponseType::DbStats => {
    //             let response = buf.splitn(2, |b| *b == SPACE_CHAR).skip(1).next().unwrap();
    //             Self::DbStats(DbStatsResponse::parse(&response).unwrap())
    //         }
    //         ResponseType::Results => {
    //             let response = buf.splitn(2, |b| *b == SPACE_CHAR).skip(1).next().unwrap();
    //             Self::Results(response.to_vec())
    //         }
    //     }
    // }
}

#[derive(Debug, Deserialize)]
pub struct DbStatsResponse {
    staff: usize,
    vn: usize,
    chars: usize,
    traits: usize,
    producers: usize,
    tags: usize,
    releases: usize,
}

impl DbStatsResponse {
    fn parse(buf: &[u8]) -> Result<Self, serde_json::Error> {
        let error: DbStatsResponse = serde_json::from_slice(&buf)?;
        println!("{:#?}", error);
        Ok(error)
    }
}
