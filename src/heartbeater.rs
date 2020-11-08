use crate::types::Heartbeat;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub fn generate_heartbeat() -> Heartbeat {
    return Heartbeat {
        status: String::from("OK"),
        version: String::from(VERSION)
    };
}