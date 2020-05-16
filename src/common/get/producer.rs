use super::GetFlag;
use super::{GetFlag::*, Results};
use serde::Deserialize;

/// All valid flags for get producer method
pub const PRODUCER_FLAGS: [GetFlag; 3] = [Basic, Details, Relations];

/// Results returned from get producer method
#[derive(Deserialize, Debug, PartialEq)]
pub struct GetProducerResults {
    #[serde(flatten)]
    pub results: Results,
    pub items: Vec<GetProducerResponse>,
}

/// All fields returned by get producer method
/// fields are either Some or None depending on GetFlag param passed to get function
#[derive(Deserialize, Debug, PartialEq)]
pub struct GetProducerResponse {
    pub id: usize,
    pub name: Option<String>,
    #[serde(rename = "original")]
    pub original_name: Option<String>,
    #[serde(rename = "type")]
    pub producer_type: Option<String>,
    pub language: Option<String>,
    pub links: Option<Links>,
    // TODO List of alternative names, separated by a newline
    pub aliases: Option<String>,
    pub description: Option<String>,
    pub relations: Option<Vec<Relations>>,
}

/// External links
#[derive(Deserialize, Debug, PartialEq)]
pub struct Links {
    pub homepage: Option<String>,
    pub wikipedia: Option<String>,
    pub wikidata: Option<String>,
}

/// List of related producers
#[derive(Deserialize, Debug, PartialEq)]
pub struct Relations {
    pub id: usize,
    pub relation: String,
    pub name: String,
    #[serde(rename = "original")]
    pub original_name: Option<String>,
}
