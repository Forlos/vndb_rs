use super::GetFlag;
use super::{GetFlag::*, Results};
use serde::Deserialize;

/// All valid flags for get ulist method
pub const ULIST_FLAGS: [GetFlag; 2] = [Basic, Labels];

/// Results returned from get ulist method
#[derive(Deserialize, Debug)]
pub struct GetUListResults {
    #[serde(flatten)]
    results: Results,
    items: Vec<GetUListResponse>,
}

/// All fields returned by get ulist method
/// fields are either Some or None depending on GetFlag param passed to get function
#[derive(Deserialize, Debug)]
pub struct GetUListResponse {
    #[serde(rename = "uid")]
    user_id: usize,
    #[serde(rename = "vn")]
    vn_id: usize,
    /// Unix timestamp of when this item has been added.
    added: usize,
    /// Unix timestamp of when this item has been added.
    lastmod: usize,
    /// Unix timestamp when the vote has been cast.
    voted: Option<usize>,
    /// Vote between 10 and 100.
    vote: Option<usize>,
    notes: Option<String>,
    started: Option<String>,
    finished: Option<String>,
    labels: Option<Vec<Label>>,
}

#[derive(Deserialize, Debug)]
pub struct Label {
    id: usize,
    label: String,
}
