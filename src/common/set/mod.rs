pub mod ulist;

use super::error::{VndbError, VndbResult};
use strum_macros::AsRefStr;
use ulist::UListFields;

#[derive(AsRefStr, Debug)]
#[strum(serialize_all = "lowercase")]
pub enum SetType {
    Ulist,
}

#[derive(Debug)]
pub(crate) enum Fields {
    Ulist(UListFields),
}

#[derive(Debug)]
pub(crate) struct SetRequest {
    set_type: SetType,
    id: usize,
    fields: Option<Fields>,
}

impl SetRequest {
    pub fn new(set_type: SetType, id: usize, fields: Option<Fields>) -> Self {
        Self {
            set_type,
            id,
            fields,
        }
    }
    pub(crate) fn to_request(&self) -> VndbResult<String> {
        let fields = match &self.fields {
            Some(Fields::Ulist(ulist)) => match serde_json::to_string(&ulist) {
                Ok(de) => de,
                Err(err) => {
                    return Err(VndbError::Other {
                        msg: err.to_string(),
                    })
                }
            },
            None => String::default(),
        };
        Ok(format!(
            "{} {} {}",
            self.set_type.as_ref(),
            &self.id,
            fields
        ))
    }
}
