pub mod character;
pub mod producer;
pub mod release;
pub mod staff;
pub mod ulist;
pub mod ulistlabels;
pub mod user;
pub mod vn;

use super::error::{VndbError, VndbResult};
use character::GetCharacterResponse;
use producer::GetProducerResponse;
use release::GetReleaseResponse;
use serde::{Deserialize, Serialize};
use serde_repr::Deserialize_repr;
use staff::GetStaffResponse;
use strum_macros::AsRefStr;
use ulist::GetUListResponse;
use ulistlabels::GetUListLabelsResponse;
use user::GetUserResponse;
use vn::GetVnResponse;

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum GetResponse {
    Vn(GetVnResponse),
    Release(GetReleaseResponse),
    Producer(GetProducerResponse),
    Character(GetCharacterResponse),
    Staff(GetStaffResponse),
    User(GetUserResponse),
    UListLabels(GetUListLabelsResponse),
    UList(GetUListResponse),
}

/// Describes number of items return
#[derive(Deserialize, Debug, PartialEq)]
pub struct Results {
    pub num: usize,
    pub more: bool,
}

#[derive(AsRefStr, Debug)]
#[strum(serialize_all = "lowercase")]
pub enum GetType {
    Vn,
    Release,
    Producer,
    Character,
    Staff,
    User,
    #[strum(serialize = "ulist-labels")]
    UlistLabels,
    Ulist,
}

#[derive(AsRefStr, Clone, Copy, Debug)]
#[strum(serialize_all = "lowercase")]
pub enum GetFlag {
    Basic,
    Details,
    Anime,
    Relations,
    Tags,
    Stats,
    Screens,
    Staff,
    Vn,
    Producers,
    #[strum(serialize = "meas")]
    Measures,
    Traits,
    Vns,
    Voiced,
    Instances,
    Aliases,
    Labels,
}

#[derive(Debug)]
pub(crate) struct GetRequest<'a> {
    get_type: GetType,
    flags: &'a [GetFlag],
    filters: String,
    options: Option<Options>,
}

impl<'a> GetRequest<'a> {
    pub fn new(
        get_type: GetType,
        flags: &'a [GetFlag],
        filters: String,
        options: Option<Options>,
    ) -> Self {
        Self {
            get_type,
            flags,
            filters,
            options,
        }
    }
    pub(crate) fn to_request(&self) -> VndbResult<String> {
        let mut flags = String::new();
        for flag in self.flags {
            flags += flag.as_ref();
            flags += ",";
        }
        let options = match &self.options {
            Some(o) => match serde_json::to_string(&o) {
                Ok(de) => de,
                Err(err) => {
                    return Err(VndbError::Other {
                        msg: err.to_string(),
                    })
                }
            },
            None => String::default(),
        };
        Ok(format!(
            "{} {} {} {}",
            self.get_type.as_ref(),
            flags,
            self.filters,
            options
        ))
    }
}

#[derive(Serialize, Debug)]
pub struct Options {
    page: usize,
    results: usize,
    sort: String,
    reverse: bool,
}

impl Options {
    pub fn new(page: usize, results: usize, sort: String, reverse: bool) -> Self {
        Self {
            page,
            results,
            sort,
            reverse,
        }
    }
}

#[repr(u8)]
#[derive(Deserialize_repr, Debug, PartialEq)]
pub enum SpoilerLevel {
    None = 0,
    Minor = 1,
    Major = 2,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Gender {
    M,
    F,
    Both,
}
