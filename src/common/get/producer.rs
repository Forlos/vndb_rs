use super::GetFlag;
use super::{GetFlag::*, Results};
use serde::Deserialize;

/// All valid flags for get producer method
pub const PRODUCER_FLAGS: [GetFlag; 3] = [Basic, Details, Relations];

/// Results returned from get producer method
#[derive(Deserialize, Debug)]
pub struct GetProducerResults {
    #[serde(flatten)]
    results: Results,
    items: Vec<GetProducerResponse>,
}

/// All fields returned by get producer method
/// fields are either Some or None depending on GetFlag param passed to get function
#[derive(Deserialize, Debug)]
pub struct GetProducerResponse {
    id: usize,
    name: Option<String>,
    #[serde(rename = "original")]
    original_name: Option<String>,
    #[serde(rename = "type")]
    producer_type: Option<String>,
    language: Option<String>,
    links: Option<Links>,
    // TODO List of alternative names, separated by a newline
    aliases: Option<String>,
    description: Option<String>,
    relations: Option<Vec<Relations>>,
}

/// External links
#[derive(Deserialize, Debug)]
pub struct Links {
    homepage: Option<String>,
    wikipedia: Option<String>,
    wikidata: Option<String>,
}

/// List of related producers
#[derive(Deserialize, Debug)]
pub struct Relations {
    id: usize,
    relation: String,
    name: String,
    #[serde(rename = "original")]
    original_name: Option<String>,
}
