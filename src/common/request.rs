use strum_macros::AsRefStr;

#[derive(AsRefStr, Debug)]
#[strum(serialize_all = "lowercase")]
pub enum RequestType {
    Login,
    DbStats,
    Get,
    Set,
}
