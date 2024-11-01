use misc::enums;
use crate::misc;

#[derive(Clone)]
pub struct Session {
    id: u32,
}

impl Session {
    pub fn close(&self, code: enums::WsCloseCode, reason: &str) {
        println!("TODO: session closed")
    }
}
