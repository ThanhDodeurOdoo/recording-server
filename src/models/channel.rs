use serde::{Serialize};
use crate::models::remote::Remote;

#[derive(Clone)]
pub struct Channel {
    pub uuid: String,
    pub recording_uuid: String,
    pub remote: Remote,
    pub remote_address: String,
    pub issuer: String,
    pub key: Option<String>,
    pub router: String, // should be a router type or a transport type
}
