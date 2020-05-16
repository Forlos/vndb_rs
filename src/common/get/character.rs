use super::GetFlag;
use super::{Gender, GetFlag::*, Results, SpoilerLevel};
use serde::Deserialize;

/// All valid flags for get character method
pub const CHARACTER_FLAGS: [GetFlag; 7] =
    [Basic, Details, Measures, Traits, Vns, Voiced, Instances];

/// Results returned from get character method
#[derive(Deserialize, Debug, PartialEq)]
pub struct GetCharacterResults {
    #[serde(flatten)]
    pub results: Results,
    pub items: Vec<GetCharacterResponse>,
}

/// All fields returned by get character method
/// fields are either Some or None depending on GetFlag param passed to get function
#[derive(Deserialize, Debug, PartialEq)]
pub struct GetCharacterResponse {
    pub id: usize,
    pub name: Option<String>,
    #[serde(rename = "original")]
    pub original_name: Option<String>,
    pub gender: Option<Gender>,
    #[serde(rename = "bloodt")]
    pub blood_type: Option<BloodType>,
    // TODO Deserialize to struct
    pub birthday: Option<Vec<Option<u8>>>,
    // TODO List of alternative names, separated by a newline
    pub aliases: Option<String>,
    pub description: Option<String>,
    #[serde(rename = "image")]
    pub image_url: Option<String>,
    pub bust: Option<usize>,
    pub waist: Option<usize>,
    pub hip: Option<usize>,
    pub height: Option<usize>,
    pub weight: Option<usize>,
    // TODO Deserialize to struct
    pub traits: Option<Vec<Vec<usize>>>,
    // TODO Deserialize to struct
    pub vns: Option<Vec<(usize, usize, SpoilerLevel, String)>>,
    pub voiced: Option<Vec<VA>>,
    pub instances: Option<Vec<Instances>>,
}

/// Blood type, "a", "b", "ab" or "o"
#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum BloodType {
    A,
    B,
    Ab,
    O,
}

/// Character role in vn
#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    Main,
    Primary,
    Side,
    Appears,
}

/// Voice actresses (staff) that voiced this character, per VN.
#[derive(Deserialize, Debug, PartialEq)]
pub struct VA {
    id: usize,
    #[serde(rename = "aid")]
    alias_id: usize,
    #[serde(rename = "vid")]
    vn_id: usize,
    note: Option<String>,
}

/// Instances of this character (excluding the character entry itself).
#[derive(Deserialize, Debug, PartialEq)]
pub struct Instances {
    id: usize,
    spoiler: SpoilerLevel,
    name: String,
    #[serde(rename = "original")]
    original_name: String,
}
