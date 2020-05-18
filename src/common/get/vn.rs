use super::GetFlag;
use super::{GetFlag::*, Results, SpoilerLevel};
use serde::Deserialize;
use serde_repr::Deserialize_repr;

/// All valid flags for get vn method
pub const VN_FLAGS: [GetFlag; 8] = [
    Basic, Details, Anime, Relations, Tags, Stats, Screens, Staff,
];

/// Results returned from get vn method
#[derive(Deserialize, Debug, PartialEq)]
pub struct GetVnResults {
    #[serde(flatten)]
    pub results: Results,
    pub items: Vec<GetVnResponse>,
}

/// All fields returned by get vn method
/// fields are either Some or None depending on GetFlag param passed to get function
#[derive(Deserialize, Debug, PartialEq)]
pub struct GetVnResponse {
    pub title: Option<String>,
    #[serde(rename = "original")]
    pub original_title: Option<String>,
    pub released: Option<String>,
    pub languages: Option<Vec<String>>,
    #[serde(rename = "orig_lang")]
    pub original_language: Option<Vec<String>>,
    pub platforms: Option<Vec<String>>,
    // TODO List of alternative names, separated by a newline
    pub aliases: Option<String>,
    pub length: Option<VnLength>,
    pub description: Option<String>,
    pub links: Option<Links>,
    pub image: Option<String>,
    pub image_nsfw: Option<bool>,
    pub anime: Option<Vec<Anime>>,
    pub relations: Option<Vec<Relations>>,
    // TODO Custom deserialize to Tag struct
    pub tags: Option<Vec<(usize, f64, SpoilerLevel)>>,
    pub popularity: Option<f64>,
    pub rating: Option<f64>,
    pub votecount: Option<usize>,
    pub screens: Option<Vec<Screens>>,
    pub staff: Option<Vec<Staff>>,
    pub id: usize,
}

/// Represents VN length
#[repr(u8)]
#[derive(Deserialize_repr, Debug, PartialEq)]
pub enum VnLength {
    VeryShort = 1,
    Short = 2,
    Medium = 3,
    Long = 4,
    VeryLong = 5,
}

/// External site links
#[derive(Deserialize, Debug, PartialEq)]
pub struct Links {
    pub wikipedia: Option<String>,
    pub encubed: Option<String>,
    pub renai: Option<String>,
    pub wikidata: Option<String>,
}

/// Anime related to the VN,
#[derive(Deserialize, Debug, PartialEq)]
pub struct Anime {
    pub id: usize,
    pub ann_id: Option<usize>,
    pub nfo_id: Option<String>,
    pub title_romaji: Option<String>,
    pub title_kanji: Option<String>,
    pub year: Option<usize>,
    #[serde(rename = "type")]
    pub typ: Option<String>,
}

/// Related visual novel
#[derive(Deserialize, Debug, PartialEq)]
pub struct Relations {
    pub id: usize,
    pub relation: String,
    pub title: String,
    #[serde(rename = "original")]
    pub original_title: Option<String>,
    pub official: bool,
}

/// Tag linked to this VN.
#[derive(Debug, PartialEq)]
pub struct Tag {
    pub id: usize,
    pub score: f64,
    pub spoiler_level: SpoilerLevel,
}

/// VN screenshots
#[derive(Deserialize, Debug, PartialEq)]
pub struct Screens {
    #[serde(rename = "image")]
    pub image_url: String,
    #[serde(rename = "rid")]
    pub release_id: usize,
    pub nsfw: bool,
    pub height: usize,
    pub width: usize,
}

/// Staff related to the VN
#[derive(Deserialize, Debug, PartialEq)]
pub struct Staff {
    #[serde(rename = "sid")]
    pub id: usize,
    #[serde(rename = "aid")]
    pub alias_id: usize,
    pub name: String,
    #[serde(rename = "original")]
    pub original_name: Option<String>,
    pub role: String,
    pub note: Option<String>,
}
