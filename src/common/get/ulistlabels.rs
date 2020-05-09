use super::GetFlag;
use super::{GetFlag::*, Results};
use serde::Deserialize;

/// All valid flags for get ulist-labels method
pub const ULIST_LABELS_FLAGS: [GetFlag; 1] = [Basic];

/// Results returned from get ulist-labels method
#[derive(Deserialize, Debug)]
pub struct GetUListLabelsResults {
    #[serde(flatten)]
    results: Results,
    items: Vec<GetUListLabelsResponse>,
}

/// All fields returned by get ulist-labels method
/// fields are either Some or None depending on GetFlag param passed to get function
#[derive(Deserialize, Debug)]
pub struct GetUListLabelsResponse {
    #[serde(rename = "uid")]
    user_id: usize,
    #[serde(rename = "id")]
    label_id: Option<usize>,
    label: Option<String>,
    private: Option<bool>,
}
