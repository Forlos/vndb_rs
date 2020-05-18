use serde::{Deserialize, Serialize};

pub type VndbResult<T> = Result<T, VndbError>;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(tag = "id", rename_all = "lowercase")]
pub enum VndbError {
    Parse {
        msg: String,
    },
    Missing {
        msg: String,
        field: String,
    },
    BadArg {
        msg: String,
        field: String,
    },
    NeedLogin {
        msg: String,
    },
    Throttled {
        msg: String,
        typ: String,
        minwait: f64,
        fullwait: f64,
    },
    Auth {
        msg: String,
    },
    LoggedIn {
        msg: String,
    },
    GetType {
        msg: String,
    },
    GetInfo {
        msg: String,
        flag: String,
    },
    Filter {
        msg: String,
        field: String,
        op: String,
        value: String,
    },
    SetType {
        msg: String,
    },
    IO {
        msg: String,
    },
    Other {
        msg: String,
    },
}

impl VndbError {
    pub(crate) fn parse_error(buf: &[u8]) -> Result<Self, serde_json::Error> {
        let error: VndbError = serde_json::from_slice(&buf)?;
        println!("{:#?}", error);
        Ok(error)
    }
}
