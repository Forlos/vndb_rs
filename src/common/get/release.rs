use super::GetFlag;
use super::{GetFlag::*, Results};
use serde::Deserialize;
use serde_repr::Deserialize_repr;

/// All valid flags for get release method
pub const RELEASE_FLAGS: [GetFlag; 4] = [Basic, Details, Vn, Producers];

/// Results returned from get release method
#[derive(Deserialize, Debug)]
pub struct GetReleaseResults {
    #[serde(flatten)]
    results: Results,
    items: Vec<GetReleaseResponse>,
}

/// All fields returned by get release method
/// fields are either Some or None depending on GetFlag param passed to get function
#[derive(Deserialize, Debug)]
pub struct GetReleaseResponse {
    id: usize,
    title: Option<String>,
    #[serde(rename = "original")]
    original_title: Option<String>,
    released: Option<String>,
    #[serde(rename = "type")]
    release_type: Option<ReleaseType>,
    patch: Option<bool>,
    freeware: Option<bool>,
    doujin: Option<bool>,
    languages: Option<Vec<String>>,
    website: Option<String>,
    notes: Option<String>,
    #[serde(rename = "minage")]
    age_rating: Option<u8>,
    #[serde(rename = "gtin")]
    barcode: Option<String>,
    catalog: Option<String>,
    platforms: Option<Vec<String>>,
    media: Option<Vec<Media>>,
    // TODO deserialize to struct
    resolution: Option<Option<String>>,
    voiced: Option<Voiced>,
    // TODO deserialize to struct
    animation: Option<Vec<Option<Animation>>>,
    vn: Option<Vec<Vn>>,
    producers: Option<Vec<Producers>>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum ReleaseType {
    Complete,
    Partial,
    Trial,
}

#[derive(Deserialize, Debug)]
pub struct Media {
    medium: String,
    #[serde(rename = "qty")]
    quantity: Option<usize>,
}

#[repr(u8)]
#[derive(Deserialize_repr, Debug)]
pub enum Voiced {
    NotVoiced = 1,
    OnlyEro = 2,
    PartVoiced = 3,
    FullVoiced = 4,
}

#[repr(u8)]
#[derive(Deserialize_repr, Debug)]
pub enum Animation {
    NoAnimations = 1,
    SimpleAnimations = 2,
    SomeFullyAnimatedScenes = 3,
    FullyAnimated = 4,
}

#[derive(Deserialize, Debug)]
pub struct Vn {
    id: usize,
    title: String,
    #[serde(rename = "original")]
    original_title: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Producers {
    id: usize,
    developer: bool,
    publisher: bool,
    name: String,
    #[serde(rename = "original")]
    original_name: Option<String>,
    #[serde(rename = "type")]
    producer_type: String,
}
