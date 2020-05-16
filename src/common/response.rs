use super::{dbstats::DbStatsResponse, error::VndbResult};
use crate::{common::error::VndbError, END_OF_TRANSMISSION, SPACE_CHAR};

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
                Self::Results(response.into())
            }
            _ => todo!(),
        })
    }
}
