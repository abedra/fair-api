use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Heartbeat {
    pub status: String,
    pub version: String
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct Error {
    pub error: String
}
