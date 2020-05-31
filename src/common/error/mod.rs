use serde::{Deserialize, Serialize};
use serde_json::Value;
use thiserror::Error;

pub type VndbResult<T> = Result<T, VndbError>;

#[derive(Serialize, Deserialize, Debug, PartialEq, Error, Clone)]
#[serde(tag = "id", rename_all = "lowercase")]
pub enum VndbError {
    #[error("{msg:}")]
    Parse { msg: String },
    #[error("{msg:} {field:}")]
    Missing { msg: String, field: String },
    #[error("{msg:} {field:}")]
    BadArg { msg: String, field: String },
    #[error("{msg:}")]
    NeedLogin { msg: String },
    #[error("{msg:} {typ:} {minwait:} {fullwait:}")]
    Throttled {
        msg: String,
        typ: String,
        minwait: f64,
        fullwait: f64,
    },
    #[error("{msg:}")]
    Auth { msg: String },
    #[error("{msg:}")]
    LoggedIn { msg: String },
    #[error("{msg:}")]
    GetType { msg: String },
    #[error("{msg:} {flag:}")]
    GetInfo { msg: String, flag: String },
    #[error("{msg:} {field:} {op:} {value:}")]
    Filter {
        msg: String,
        field: String,
        op: String,
        value: Value,
    },
    #[error("{msg:}")]
    SetType { msg: String },
    #[error("{msg:}")]
    IO { msg: String },
    #[error("{msg:}")]
    Other { msg: String },
}

impl VndbError {
    pub(crate) fn parse_error(buf: &[u8]) -> Result<Self, serde_json::Error> {
        let error: VndbError = serde_json::from_slice(&buf)?;
        Ok(error)
    }
}
