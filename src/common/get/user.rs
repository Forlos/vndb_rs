use super::GetFlag;
use super::{GetFlag::*, Results};
use serde::Deserialize;

/// All valid flags for get user method
pub const USER_FLAGS: [GetFlag; 1] = [Basic];

/// Results returned from get user method
#[derive(Deserialize, Debug, PartialEq)]
pub struct GetUserResults {
    #[serde(flatten)]
    pub results: Results,
    pub items: Vec<GetUserResponse>,
}

/// All fields returned by get user method
/// fields are either Some or None depending on GetFlag param passed to get function
#[derive(Deserialize, Debug, PartialEq)]
pub struct GetUserResponse {
    pub id: usize,
    pub username: Option<String>,
}
