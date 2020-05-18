use strum_macros::AsRefStr;

#[derive(AsRefStr, Debug)]
#[strum(serialize_all = "lowercase")]
pub(crate) enum RequestType {
    Login,
    DbStats,
    Get,
    Set,
}
