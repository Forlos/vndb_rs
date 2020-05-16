use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DbStatsResponse {
    pub staff: usize,
    pub vn: usize,
    pub chars: usize,
    pub traits: usize,
    pub producers: usize,
    pub tags: usize,
    pub releases: usize,
}

impl DbStatsResponse {
    pub(crate) fn parse(buf: &[u8]) -> Result<Self, serde_json::Error> {
        let response: DbStatsResponse = serde_json::from_slice(&buf)?;
        Ok(response)
    }
}
