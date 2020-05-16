use super::GetFlag;
use super::{GetFlag::*, Results};
use serde::Deserialize;

/// All valid flags for get ulist-labels method
pub const ULIST_LABELS_FLAGS: [GetFlag; 1] = [Basic];

/// Results returned from get ulist-labels method
#[derive(Deserialize, Debug, PartialEq)]
pub struct GetUListLabelsResults {
    #[serde(flatten)]
    pub results: Results,
    pub items: Vec<GetUListLabelsResponse>,
}

/// All fields returned by get ulist-labels method
/// fields are either Some or None depending on GetFlag param passed to get function
#[derive(Deserialize, Debug, PartialEq)]
pub struct GetUListLabelsResponse {
    #[serde(rename = "uid")]
    pub user_id: usize,
    #[serde(rename = "id")]
    pub label_id: Option<usize>,
    pub label: Option<String>,
    pub private: Option<bool>,
}
