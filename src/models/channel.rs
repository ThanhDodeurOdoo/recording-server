use std::collections::HashMap;
use once_cell::sync::Lazy;
use std::sync::Arc;
use tokio::sync::Mutex;

use uuid::Uuid;
use crate::models;
use serde::{Serialize, Deserialize};

use models::session::{ Session };
use crate::state::{CHANNELS};

pub type ChannelMap = Arc<Mutex<HashMap<String, Arc<Channel>>>>;
pub static CHANNEL_BY_ISSUER: Lazy<ChannelMap> = Lazy::new(|| {
    Arc::new(Mutex::new(HashMap::new()))
});

#[derive(Clone)]
pub struct Channel {
    pub uuid: String,
    pub remote_address: String,
    issuer: String,
    key: Option<String>,
    pub sessions: HashMap<u32, Session>,
}

impl Channel {
    pub async fn create(
        remote_address: &str,
        issuer: &str,
        key: Option<String>,
        use_web_rtc: bool,
    ) -> Arc<Channel> {
        let uuid = Uuid::new_v4().to_string();

        // Check if a channel with this issuer already exists
        let mut channels_by_issuer = CHANNEL_BY_ISSUER.lock().await;
        if let Some(existing_channel) = channels_by_issuer.get(issuer) {
            return existing_channel.clone();
        }

        // If no channel exists, create a new one and wrap it in Arc
        let channel = Arc::new(Channel {
            uuid: uuid.clone(),
            remote_address: remote_address.to_string(),
            issuer: issuer.to_string(),
            key,
            sessions: HashMap::new(),
        });

        // Insert the new channel into both CHANNEL_BY_ISSUER and CHANNELS
        channels_by_issuer.insert(issuer.to_string(), channel.clone());

        let mut channels = CHANNELS.lock().await;
        channels.insert(uuid.clone(), channel.clone());

        // Return the newly created channel wrapped in Arc
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