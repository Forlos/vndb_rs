use derive_builder::Builder;
use serde::Serialize;
use strum_macros::AsRefStr;

#[derive(AsRefStr, Debug)]
#[strum(serialize_all = "lowercase")]
pub enum SetType {
    Ulist,
}

#[derive(Debug)]
pub(crate) enum Fields {
    Ulist(UListFields),
}

#[builder(default, setter(into))]
#[derive(Serialize, Default, Builder, Debug)]
pub struct UListFields {
    /// User-set notes
    notes: Option<String>,
    /// YYYY-MM-DD
    started: Option<String>,
    /// YYYY-MM-DD
    finished: Option<String>,
    /// Vote between 10 and 100.
    vote: Option<usize>,
    /// List of label IDs to assign to this VN. This will overwrite any previously assigned labels.
    labels: Vec<usize>,
}

#[derive(Debug)]
pub(crate) struct SetRequest {
    pub(crate) set_type: SetType,
    pub(crate) id: String,
    pub(crate) fields: Option<Fields>,
}

impl SetRequest {
    pub(crate) fn to_request(&self) -> String {
        let fields = match &self.fields {
            Some(Fields::Ulist(ulist)) => serde_json::to_string(&ulist).unwrap(),
            None => String::default(),
        };
        format!("{} {} {}", self.set_type.as_ref(), &self.id, fields)
    }
}
