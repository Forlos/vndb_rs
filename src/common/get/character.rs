use super::GetFlag;
use super::{Gender, GetFlag::*, Results, SpoilerLevel};
use serde::Deserialize;

/// All valid flags for get character method
pub const CHARACTER_FLAGS: [GetFlag; 7] =
    [Basic, Details, Measures, Traits, Vns, Voiced, Instances];

/// Results returned from get character method
#[derive(Deserialize, Debug)]
pub struct GetCharacterResults {
    #[serde(flatten)]
    results: Results,
    items: Vec<GetCharacterResponse>,
}

/// All fields returned by get character method
/// fields are either Some or None depending on GetFlag param passed to get function
#[derive(Deserialize, Debug)]
pub struct GetCharacterResponse {
    id: usize,
    name: Option<String>,
    #[serde(rename = "original")]
    original_name: Option<String>,
    gender: Option<Gender>,
    #[serde(rename = "bloodt")]
    blood_type: Option<BloodType>,
    // TODO Deserialize to struct
    birthday: Option<Vec<Option<u8>>>,
    // TODO List of alternative names, separated by a newline
    aliases: Option<String>,
    description: Option<String>,
    #[serde(rename = "image")]
    image_url: Option<String>,
    bust: Option<usize>,
    waist: Option<usize>,
    hip: Option<usize>,
    height: Option<usize>,
    weight: Option<usize>,
    // TODO Deserialize to struct
    traits: Option<Vec<Vec<usize>>>,
    // TODO Deserialize to struct
    vns: Option<Vec<(usize, usize, SpoilerLevel, String)>>,
    voiced: Option<Vec<VA>>,
    instances: Option<Vec<Instances>>,
}

/// Blood type, "a", "b", "ab" or "o"
#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum BloodType {
    A,
    B,
    Ab,
    O,
}

/// Character role in vn
#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    Main,
    Primary,
    Side,
    Appears,
}

/// Voice actresses (staff) that voiced this character, per VN.
#[derive(Deserialize, Debug)]
pub struct VA {
    id: usize,
    #[serde(rename = "aid")]
    alias_id: usize,
    #[serde(rename = "vid")]
    vn_id: usize,
    note: Option<String>,
}

/// Instances of this character (excluding the character entry itself).
#[derive(Deserialize, Debug)]
pub struct Instances {
    id: usize,
    spoiler: SpoilerLevel,
    name: String,
    #[serde(rename = "original")]
    original_name: String,
}
