use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

use crate::models;
use serde::{Serialize, Deserialize};
use models::session::{ Session };
pub type SharedChannels = Arc<Mutex<HashMap<String, Channel>>>;

#[derive(Clone)]
pub struct Channel {
    pub uuid: String,
    pub remote_address: String,
    issuer: String,
    key: Option<String>,
    pub sessions: HashMap<u32, Session>,
}

impl Channel {
    pub fn create(
        remote_address: &str,
        issuer: &str,
        key: Option<String>,
        use_web_rtc: bool,
        channels: SharedChannels,
    ) -> Self {
        let uuid = Uuid::new_v4().to_string();
        let channel = Channel {
            uuid: uuid.clone(),
            remote_address: remote_address.to_string(),
            issuer: issuer.to_string(),
            key,
            sessions: HashMap::new(),
        };
        channels.lock().unwrap().insert(uuid, channel.clone());
        channel
    }

    pub fn get_stats(&self) -> ChannelStats {
        ChannelStats {
            uuid: self.uuid.clone(),
        }
    }
}

#[derive(Serialize)]
pub struct ChannelStats {
    uuid: String,
}