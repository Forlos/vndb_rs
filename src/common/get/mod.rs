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
pub(crate) enum GetResponse {
    Vn(Box<GetVnResponse>),
    Release(Box<GetReleaseResponse>),
    Producer(Box<GetProducerResponse>),
    Character(Box<GetCharacterResponse>),
    Staff(Box<GetStaffResponse>),
    User(Box<GetUserResponse>),
    UListLabels(Box<GetUListLabelsResponse>),
    UList(Box<GetUListResponse>),
}

/// Describes number of items return
#[derive(Deserialize, Debug, PartialEq)]
pub struct Results {
    pub num: usize,
    pub more: bool,
}

#[derive(AsRefStr, Debug)]
#[strum(serialize_all = "lowercase")]
pub(crate) enum GetType {
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

/// The options argument is optional, and influences the behaviour of the returned results.
#[derive(Serialize, Debug)]
pub struct Options {
    /// Page 1 (the default) returns the first 10 results (1-10), page 2 returns the following 10 (11-20), etc.
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<usize>,
    /// Maximum number of results to return.
    /// Also affects the "page" option above.
    /// For example: with "page" set to 2 and "results" set to 5, the second five results (that is, results 6-10) will be returned.
    /// Default: 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    results: Option<usize>,
    /// The field to order the results by.
    /// The accepted field names differ per type.
    /// The default sort field is the ID of the database entry.
    #[serde(skip_serializing_if = "Option::is_none")]
    sort: Option<String>,
    /// Set to true to reverse the order of the results.
    /// Default false.
    #[serde(skip_serializing_if = "Option::is_none")]
    reverse: Option<bool>,
}

impl Options {
    pub fn new(
        page: Option<usize>,
        results: Option<usize>,
        sort: Option<String>,
        reverse: Option<bool>,
    ) -> Self {
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
