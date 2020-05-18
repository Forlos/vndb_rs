use super::GetFlag;
use super::{Gender, GetFlag::*, Results};
use serde::Deserialize;

/// All valid flags for get staff method
pub const STAFF_FLAGS: [GetFlag; 5] = [Basic, Details, Aliases, Vns, Voiced];

/// Results returned from get staff method
#[derive(Deserialize, Debug, PartialEq)]
pub struct GetStaffResults {
    #[serde(flatten)]
    pub results: Results,
    pub items: Vec<GetStaffResponse>,
}

/// All fields returned by get staff method
/// fields are either Some or None depending on GetFlag param passed to get function
#[derive(Deserialize, Debug, PartialEq)]
pub struct GetStaffResponse {
    pub id: usize,
    pub name: Option<String>,
    #[serde(rename = "original")]
    pub original_name: Option<String>,
    pub gender: Option<Gender>,
    pub language: Option<String>,
    pub links: Option<Links>,
    pub description: Option<String>,
    // TODO Deserialize to struct
    pub aliases: Option<Vec<(usize, String, String)>>,
    pub main_alias: Option<usize>,
    pub vns: Option<Vec<Vn>>,
    pub voiced: Option<Vec<CV>>,
}

/// External site links
#[derive(Deserialize, Debug, PartialEq)]
pub struct Links {
    pub homepage: Option<String>,
    pub wikipedia: Option<String>,
    pub twitter: Option<String>,
    #[serde(rename = "anidb")]
    pub anidb_id: Option<usize>,
    pub pixiv: Option<String>,
    pub wikidata: Option<String>,
}

/// Visual novels that this staff entry has been credited in (excluding character voicing).
#[derive(Deserialize, Debug, PartialEq)]
pub struct Vn {
    pub id: usize,
    #[serde(rename = "aid")]
    pub alias_id: usize,
    pub role: String,
    pub note: Option<String>,
}

/// Characters that this staff entry has voiced.
#[derive(Deserialize, Debug, PartialEq)]
pub struct CV {
    #[serde(rename = "id")]
    pub vn_id: usize,
    #[serde(rename = "aid")]
    pub alias_id: usize,
    #[serde(rename = "cid")]
    pub character_id: usize,
    pub note: Option<String>,
}
