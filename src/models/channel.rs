use uuid::Uuid;
use serde::{Serialize};

#[derive(Clone)]
pub struct Channel {
    pub uuid: String, // readonly crate?
    pub remote_address: String,
    pub issuer: String,
    pub key: Option<String>, // SFU should share the client key with the recorder
}

impl Channel {
    pub async fn create(
        remote_address: &str,
        issuer: &str,
        key: Option<String>,
    ) -> Channel {
        let uuid = Uuid::new_v4().to_string();
        Channel {
            uuid: uuid.clone(),
            remote_address: remote_address.to_string(),
            issuer: issuer.to_string(),
            key,
        }
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