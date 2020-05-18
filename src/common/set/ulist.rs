use serde::Serialize;

#[derive(Serialize, Default, Debug)]
pub struct UListFields {
    /// User-set notes
    #[serde(skip_serializing_if = "Option::is_none")]
    notes: Option<String>,
    /// YYYY-MM-DD
    #[serde(skip_serializing_if = "Option::is_none")]
    started: Option<String>,
    /// YYYY-MM-DD
    #[serde(skip_serializing_if = "Option::is_none")]
    finished: Option<String>,
    /// Vote between 10 and 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    vote: Option<usize>,
    /// List of label IDs to assign to this VN. This will overwrite any previously assigned labels.
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<Vec<usize>>,
}

impl UListFields {
    pub fn new(
        notes: Option<String>,
        started: Option<VndbDate>,
        finished: Option<VndbDate>,
        vote: Option<usize>,
        labels: Option<Vec<usize>>,
    ) -> Self {
        Self {
            notes,
            started: started.map(|date| date.to_vndb_date_string()),
            finished: finished.map(|date| date.to_vndb_date_string()),
            vote,
            labels,
        }
    }
}

#[derive(Debug)]
pub struct VndbDate {
    year: u16,
    month: u8,
    day: u8,
}

impl VndbDate {
    pub fn new(year: u16, month: u8, day: u8) -> Self {
        Self { year, month, day }
    }
    fn to_vndb_date_string(&self) -> String {
        format!("{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }
}
