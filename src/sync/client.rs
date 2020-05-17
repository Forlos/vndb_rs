use serde::de::DeserializeOwned;
use std::io::{BufRead, BufReader, Read, Write};

use crate::common::response::Response;
use crate::common::{
    dbstats::DbStatsResponse,
    error::{VndbError, VndbResult},
    get::{
        character::GetCharacterResults, producer::GetProducerResults, release::GetReleaseResults,
        staff::GetStaffResults, ulist::GetUListResults, ulistlabels::GetUListLabelsResults,
        user::GetUserResults, vn::GetVnResults, GetFlag, GetRequest, GetType, Options,
    },
    login::LoginRequest,
    request::RequestType,
    set::{ulist::UListFields, Fields, SetRequest, SetType},
};
use crate::{END_OF_TRANSMISSION, SPACE_CHAR};

#[derive(Debug)]
pub struct Client<IO>
where
    IO: Read + Write,
{
    stream: BufReader<IO>,
}

impl<IO> Client<IO>
where
    IO: Read + Write,
{
    #[inline]
    fn read(&mut self) -> VndbResult<Vec<u8>> {
        let mut buf = Vec::with_capacity(0x100);
        match self.stream.read_until(END_OF_TRANSMISSION, &mut buf) {
            Ok(_) => Ok(buf),
            Err(err) => Err(VndbError::IO {
                msg: err.to_string(),
            }),
        }
    }
    #[inline]
    fn write(&mut self, input: &[u8]) -> VndbResult<()> {
        let writer = self.stream.get_mut();
        match writer.write(&input) {
            Ok(_) => match writer.flush() {
                Ok(_) => Ok(()),
                Err(err) => Err(VndbError::IO {
                    msg: err.to_string(),
                }),
            },
            Err(err) => Err(VndbError::IO {
                msg: err.to_string(),
            }),
        }
    }
    #[inline]
    fn make_request(&mut self, request_type: RequestType, buf: &[u8]) -> VndbResult<Response> {
        let mut input = Vec::with_capacity(0x100);
        input.extend(request_type.as_ref().as_bytes());
        input.push(SPACE_CHAR);
        input.extend(buf);
        input.push(END_OF_TRANSMISSION);
        self.write(&input)?;
        Response::parse_response(&self.read()?)
    }
    #[inline]
    fn get<T>(
        &mut self,
        get_type: GetType,
        flags: &[GetFlag],
        filters: String,
        options: Option<Options>,
    ) -> VndbResult<T>
    where
        T: DeserializeOwned,
    {
        let response = self.make_request(
            RequestType::Get,
            GetRequest::new(get_type, flags, filters, options)
                .to_request()?
                .as_bytes(),
        )?;
        match response {
            Response::Results(vec) => match serde_json::from_slice(&vec) {
                Ok(de) => Ok(de),
                Err(err) => Err(VndbError::Other {
                    msg: err.to_string(),
                }),
            },
            Response::Error(err) => Err(err),
            _ => Err(VndbError::Other {
                msg: "Unexpected response type".to_owned(),
            }),
        }
    }
    #[inline]
    fn set(&mut self, set_type: SetType, id: usize, fields: Option<Fields>) -> VndbResult<()> {
        let response = self.make_request(
            RequestType::Set,
            SetRequest::new(set_type, id, fields)
                .to_request()?
                .as_bytes(),
        )?;
        match response {
            Response::Ok => Ok(()),
            Response::Error(err) => Err(err),
            _ => Err(VndbError::Other {
                msg: "Unexpected response type".to_owned(),
            }),
        }
    }

    pub fn new(stream: IO) -> Self {
        Self {
            stream: BufReader::new(stream),
        }
    }
    /// Login without credentials, using set commands will result in error
    pub fn login(&mut self) -> VndbResult<()> {
        let request = LoginRequest::default();
        let response = self.make_request(
            RequestType::Login,
            &match serde_json::to_vec(&request) {
                Ok(de) => de,
                Err(err) => {
                    return Err(VndbError::Other {
                        msg: err.to_string(),
                    })
                }
            },
        )?;
        match response {
            Response::Ok => Ok(()),
            Response::Error(err) => Err(err),
            _ => Err(VndbError::Other {
                msg: "Unexpected response type".to_owned(),
            }),
        }
    }
    /// Login using credentials, allowing using set commands
    pub fn login_with_credentials(&mut self, username: &str, password: &str) -> VndbResult<()> {
        let request = LoginRequest::new(username, password);
        let response = self.make_request(
            RequestType::Login,
            &match serde_json::to_vec(&request) {
                Ok(de) => de,
                Err(err) => {
                    return Err(VndbError::Other {
                        msg: err.to_string(),
                    })
                }
            },
        )?;
        match response {
            Response::Ok => Ok(()),
            Response::Error(err) => Err(err),
            _ => Err(VndbError::Other {
                msg: "Unexpected response type".to_owned(),
            }),
        }
    }
    /// Get vndb stats
    pub fn get_dbstats(&mut self) -> VndbResult<DbStatsResponse> {
        let response = self.make_request(RequestType::DbStats, &Vec::new())?;
        match response {
            Response::DbStats(db_stats) => Ok(db_stats),
            Response::Error(err) => Err(err),
            _ => Err(VndbError::Other {
                msg: "Unexpected response type".to_owned(),
            }),
        }
    }
    /// Get visual novels
    pub fn get_vn(
        &mut self,
        flags: &[GetFlag],
        filters: String,
        options: Option<Options>,
    ) -> VndbResult<GetVnResults> {
        self.get(GetType::Vn, flags, filters, options)
    }
    /// Get releases
    pub fn get_release(
        &mut self,
        flags: &[GetFlag],
        filters: String,
        options: Option<Options>,
    ) -> VndbResult<GetReleaseResults> {
        self.get(GetType::Release, flags, filters, options)
    }
    /// Get producers
    pub fn get_producer(
        &mut self,
        flags: &[GetFlag],
        filters: String,
        options: Option<Options>,
    ) -> VndbResult<GetProducerResults> {
        self.get(GetType::Producer, flags, filters, options)
    }
    /// Get characters
    pub fn get_character(
        &mut self,
        flags: &[GetFlag],
        filters: String,
        options: Option<Options>,
    ) -> VndbResult<GetCharacterResults> {
        self.get(GetType::Character, flags, filters, options)
    }
    /// Get staff
    pub fn get_staff(
        &mut self,
        flags: &[GetFlag],
        filters: String,
        options: Option<Options>,
    ) -> VndbResult<GetStaffResults> {
        self.get(GetType::Staff, flags, filters, options)
    }
    /// Get users
    pub fn get_user(
        &mut self,
        flags: &[GetFlag],
        filters: String,
        options: Option<Options>,
    ) -> VndbResult<GetUserResults> {
        self.get(GetType::User, flags, filters, options)
    }
    /// Get user list labels
    pub fn get_ulist_labels(
        &mut self,
        flags: &[GetFlag],
        filters: String,
        options: Option<Options>,
    ) -> VndbResult<GetUListLabelsResults> {
        self.get(GetType::UlistLabels, flags, filters, options)
    }
    /// Get user lists
    pub fn get_ulist(
        &mut self,
        flags: &[GetFlag],
        filters: String,
        options: Option<Options>,
    ) -> VndbResult<GetUListResults> {
        self.get(GetType::Ulist, flags, filters, options)
    }
    /// This command facilitates adding, removing and modifying your VN list.
    ///
    /// The id argument is the visual novel ID
    pub fn set_ulist(&mut self, id: usize, ulist: UListFields) -> VndbResult<()> {
        self.set(SetType::Ulist, id, Some(Fields::Ulist(ulist)))
    }
    /// Remove visual novel from user list
    ///
    /// When removing a ulist item,
    /// any releases associated with the VN will be removed from the users' list as well.
    pub fn delete_ulist(&mut self, id: usize) -> VndbResult<()> {
        self.set(SetType::Ulist, id, None)
    }
}
