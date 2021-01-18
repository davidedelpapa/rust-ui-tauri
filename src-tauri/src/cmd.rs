use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct GetResponsePayload {
    pub state: u64,
}

#[derive(Deserialize)]
#[serde(tag = "cmd", rename_all = "camelCase")]
pub enum Cmd {
    GetResponse {
        payload: GetResponsePayload,
        callback: String,
        error: String,
    },
}

#[derive(Serialize)]
pub struct Response<'a> {
    pub message: &'a str,
}
