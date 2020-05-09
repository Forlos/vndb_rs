use super::GetFlag;
use super::{GetFlag::*, Results, SpoilerLevel};
use serde::Deserialize;
use serde_repr::Deserialize_repr;

/// All valid flags for get vn method
pub const VN_FLAGS: [GetFlag; 8] = [
    Basic, Details, Anime, Relations, Tags, Stats, Screens, Staff,
];

/// Results returned from get vn method
#[derive(Deserialize, Debug)]
pub struct GetVnResults {
    #[serde(flatten)]
    results: Results,
    items: Vec<GetVnResponse>,
}

/// All fields returned by get vn method
/// fields are either Some or None depending on GetFlag param passed to get function
#[derive(Deserialize, Debug)]
pub struct GetVnResponse {
    title: Option<String>,
    #[serde(rename = "original")]
    original_title: Option<String>,
    released: Option<String>,
    languages: Option<Vec<String>>,
    #[serde(rename = "orig_lang")]
    original_language: Option<Vec<String>>,
    platforms: Option<Vec<String>>,
    // TODO List of alternative names, separated by a newline
    aliases: Option<String>,
    length: Option<VnLength>,
    description: Option<String>,
    links: Option<Links>,
    image: Option<String>,
    image_nsfw: Option<bool>,
    anime: Option<Vec<Anime>>,
    relations: Option<Vec<Relations>>,
    // TODO Custom deserialize to Tag struct
    tags: Option<Vec<(usize, f64, SpoilerLevel)>>,
    popularity: Option<f64>,
    rating: Option<f64>,
    votecount: Option<usize>,
    screens: Option<Vec<Screens>>,
    staff: Option<Vec<Staff>>,
    id: usize,
}

#[repr(u8)]
#[derive(Deserialize_repr, Debug)]
pub enum VnLength {
    VeryShort = 1,
    Short = 2,
    Medium = 3,
    Long = 4,
    VeryLong = 5,
}

#[derive(Deserialize, Debug)]
pub struct Links {
    wikipedia: Option<String>,
    encubed: Option<String>,
    renai: Option<String>,
    wikidata: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Anime {
    id: usize,
    ann_id: Option<usize>,
    nfo_id: Option<String>,
    title_romaji: Option<String>,
    title_kanji: Option<String>,
    year: Option<usize>,
    #[serde(rename = "type")]
    typ: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Relations {
    id: usize,
    relation: String,
    title: String,
    #[serde(rename = "original")]
    original_title: Option<String>,
    official: bool,
}

#[derive(Debug)]
struct Tag {
    id: usize,
    score: f64,
    spoiler_level: SpoilerLevel,
}

#[derive(Deserialize, Debug)]
pub struct Screens {
    #[serde(rename = "image")]
    image_url: String,
    #[serde(rename = "rid")]
    release_id: usize,
    nsfw: bool,
    height: usize,
    width: usize,
}

#[derive(Deserialize, Debug)]
pub struct Staff {
    #[serde(rename = "sid")]
    id: usize,
    #[serde(rename = "aid")]
    alias_id: usize,
    name: String,
    #[serde(rename = "original")]
    original_name: Option<String>,
    role: String,
    note: Option<String>,
}
