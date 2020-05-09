use super::GetFlag;
use super::{Gender, GetFlag::*, Results};
use serde::Deserialize;

/// All valid flags for get staff method
pub const STAFF_FLAGS: [GetFlag; 5] = [Basic, Details, Aliases, Vns, Voiced];

/// Results returned from get staff method
#[derive(Deserialize, Debug)]
pub struct GetStaffResults {
    #[serde(flatten)]
    results: Results,
    items: Vec<GetStaffResponse>,
}

/// All fields returned by get staff method
/// fields are either Some or None depending on GetFlag param passed to get function
#[derive(Deserialize, Debug)]
pub struct GetStaffResponse {
    id: usize,
    name: Option<String>,
    #[serde(rename = "original")]
    original_name: Option<String>,
    gender: Option<Gender>,
    language: Option<String>,
    links: Option<Links>,
    description: Option<String>,
    // TODO Deserialize to struct
    aliases: Option<Vec<(usize, String, String)>>,
    main_alias: Option<usize>,
    vns: Option<Vec<Vn>>,
    voiced: Option<Vec<CV>>,
}

/// External site links
#[derive(Deserialize, Debug)]
pub struct Links {
    homepage: Option<String>,
    wikipedia: Option<String>,
    twitter: Option<String>,
    #[serde(rename = "anidb")]
    anidb_id: Option<usize>,
    pixiv: Option<String>,
    wikidata: Option<String>,
}

/// Visual novels that this staff entry has been credited in (excluding character voicing).
#[derive(Deserialize, Debug)]
pub struct Vn {
    id: usize,
    #[serde(rename = "aid")]
    alias_id: usize,
    role: String,
    note: Option<String>,
}

/// Characters that this staff entry has voiced.
#[derive(Deserialize, Debug)]
pub struct CV {
    #[serde(rename = "id")]
    vn_id: usize,
    #[serde(rename = "aid")]
    alias_id: usize,
    #[serde(rename = "cid")]
    character_id: usize,
    note: Option<String>,
}
