use super::{dbstats::DbStatsResponse, error::VndbResult};
use crate::{common::error::VndbError, END_OF_TRANSMISSION, SPACE_CHAR};

#[derive(Debug)]
pub(crate) enum Response {
    Ok,
    DbStats(DbStatsResponse),
    Results(Vec<u8>),
}

impl Response {
    pub(crate) fn parse_response(buf: &[u8]) -> VndbResult<Self> {
        match buf {
            [0x6F, 0x6B, END_OF_TRANSMISSION] => Ok(Self::Ok),
            [0x65, 0x72, 0x72, 0x6f, 0x72, SPACE_CHAR, response @ .., END_OF_TRANSMISSION] => {
                match VndbError::parse_error(response) {
                    Ok(de) => Err(de),
                    Err(err) => Err(VndbError::Other {
                        msg: err.to_string(),
                    }),
                }
            }
            [0x64, 0x62, 0x73, 0x74, 0x61, 0x74, 0x73, SPACE_CHAR, response @ .., END_OF_TRANSMISSION] => {
                match DbStatsResponse::parse(response) {
                    Ok(de) => Ok(Self::DbStats(de)),
                    Err(err) => Err(VndbError::Other {
                        msg: err.to_string(),
                    }),
                }
            }
            [0x72, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x73, SPACE_CHAR, response @ .., END_OF_TRANSMISSION] => {
                Ok(Self::Results(response.into()))
            }
            _ => Err(VndbError::Other {
                msg: "Invalid response type".to_owned(),
            }),
        }
    }
}
