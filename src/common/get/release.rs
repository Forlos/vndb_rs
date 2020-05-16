use super::GetFlag;
use super::{GetFlag::*, Results};
use serde::Deserialize;
use serde_repr::Deserialize_repr;

/// All valid flags for get release method
pub const RELEASE_FLAGS: [GetFlag; 4] = [Basic, Details, Vn, Producers];

/// Results returned from get release method
#[derive(Deserialize, Debug, PartialEq)]
pub struct GetReleaseResults {
    #[serde(flatten)]
    pub results: Results,
    pub items: Vec<GetReleaseResponse>,
}

/// All fields returned by get release method
/// fields are either Some or None depending on GetFlag param passed to get function
#[derive(Deserialize, Debug, PartialEq)]
pub struct GetReleaseResponse {
    pub id: usize,
    pub title: Option<String>,
    #[serde(rename = "original")]
    pub original_title: Option<String>,
    pub released: Option<String>,
    #[serde(rename = "type")]
    pub release_type: Option<ReleaseType>,
    pub patch: Option<bool>,
    pub freeware: Option<bool>,
    pub doujin: Option<bool>,
    pub languages: Option<Vec<String>>,
    pub website: Option<String>,
    pub notes: Option<String>,
    #[serde(rename = "minage")]
    pub age_rating: Option<u8>,
    #[serde(rename = "gtin")]
    pub barcode: Option<String>,
    pub catalog: Option<String>,
    pub platforms: Option<Vec<String>>,
    pub media: Option<Vec<Media>>,
    // TODO deserialize to struct
    pub resolution: Option<Option<String>>,
    pub voiced: Option<Voiced>,
    // TODO deserialize to struct
    pub animation: Option<Vec<Option<Animation>>>,
    pub vn: Option<Vec<Vn>>,
    pub producers: Option<Vec<Producers>>,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ReleaseType {
    Complete,
    Partial,
    Trial,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct Media {
    pub medium: String,
    #[serde(rename = "qty")]
    pub quantity: Option<usize>,
}

#[repr(u8)]
#[derive(Deserialize_repr, Debug, PartialEq)]
pub enum Voiced {
    NotVoiced = 1,
    OnlyEro = 2,
    PartVoiced = 3,
    FullVoiced = 4,
}

#[repr(u8)]
#[derive(Deserialize_repr, Debug, PartialEq)]
pub enum Animation {
    NoAnimations = 1,
    SimpleAnimations = 2,
    SomeFullyAnimatedScenes = 3,
    FullyAnimated = 4,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct Vn {
    pub id: usize,
    pub title: String,
    #[serde(rename = "original")]
    pub original_title: Option<String>,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct Producers {
    pub id: usize,
    pub developer: bool,
    pub publisher: bool,
    pub name: String,
    #[serde(rename = "original")]
    pub original_name: Option<String>,
    #[serde(rename = "type")]
    pub producer_type: String,
}
