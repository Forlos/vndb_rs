use super::GetFlag;
use super::{GetFlag::*, Results};
use serde::Deserialize;

/// All valid flags for get user method
pub const USER_FLAGS: [GetFlag; 1] = [Basic];

/// Results returned from get user method
#[derive(Deserialize, Debug)]
pub struct GetUserResults {
    #[serde(flatten)]
    results: Results,
    items: Vec<GetUserResponse>,
}

/// All fields returned by get user method
/// fields are either Some or None depending on GetFlag param passed to get function
#[derive(Deserialize, Debug)]
pub struct GetUserResponse {
    id: usize,
    username: Option<String>,
}
