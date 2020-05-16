use super::GetFlag;
use super::{GetFlag::*, Results};
use serde::Deserialize;

/// All valid flags for get ulist method
pub const ULIST_FLAGS: [GetFlag; 2] = [Basic, Labels];

/// Results returned from get ulist method
#[derive(Deserialize, Debug, PartialEq)]
pub struct GetUListResults {
    #[serde(flatten)]
    pub results: Results,
    pub items: Vec<GetUListResponse>,
}

/// All fields returned by get ulist method
/// fields are either Some or None depending on GetFlag param passed to get function
#[derive(Deserialize, Debug, PartialEq)]
pub struct GetUListResponse {
    #[serde(rename = "uid")]
    pub user_id: usize,
    #[serde(rename = "vn")]
    pub vn_id: usize,
    /// Unix timestamp of when this item has been added.
    pub added: usize,
    /// Unix timestamp of when this item has been added.
    pub lastmod: usize,
    /// Unix timestamp when the vote has been cast.
    pub voted: Option<usize>,
    /// Vote between 10 and 100.
    pub vote: Option<usize>,
    pub notes: Option<String>,
    pub started: Option<String>,
    pub finished: Option<String>,
    pub labels: Option<Vec<Label>>,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct Label {
    pub id: usize,
    pub label: String,
}
