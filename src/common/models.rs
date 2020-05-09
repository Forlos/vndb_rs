use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[builder(default, setter(into, skip))]
#[derive(Serialize, Deserialize, Default, Builder, Debug)]
pub(crate) struct LoginRequest {
    #[builder(default = "1")]
    protocol: u8,
    #[builder(default = "\"vndb_rs\".to_owned()")]
    client: String,
    #[builder(default = "env!(\"CARGO_PKG_VERSION\").to_owned()")]
    clientver: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    #[builder(setter)]
    username: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    #[builder(setter)]
    password: String,
}
