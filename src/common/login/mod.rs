use serde::Serialize;

const PROTOCOL_VER: u8 = 1;
const CLIENT: &'static str = "vndb_rs";
const CLIENT_VER: &'static str = env!("CARGO_PKG_VERSION");

#[derive(Serialize, Debug)]
pub(crate) struct LoginRequest<'a> {
    protocol: u8,
    client: &'static str,
    clientver: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    username: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    password: Option<&'a str>,
}

impl<'a> Default for LoginRequest<'a> {
    fn default() -> Self {
        Self {
            protocol: PROTOCOL_VER,
            client: CLIENT,
            clientver: CLIENT_VER,
            username: None,
            password: None,
        }
    }
}

impl<'a> LoginRequest<'a> {
    pub(crate) fn new(username: &'a str, password: &'a str) -> Self {
        let mut new = Self::default();
        new.username = Some(username);
        new.password = Some(password);
        new
    }
}
