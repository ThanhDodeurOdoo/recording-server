use std::collections::HashMap;
use std::sync::{{Arc, LazyLock }};
use tokio::sync::Mutex;

use uuid::Uuid;
use crate::models;
use serde::{Serialize, Deserialize};

use models::session::{ Session };
use crate::state::{CHANNELS};

pub type ChannelMap = Arc<Mutex<HashMap<String, Arc<Channel>>>>;
pub static CHANNEL_BY_ISSUER: LazyLock<ChannelMap> = LazyLock::new(|| {
    Arc::new(Mutex::new(HashMap::new()))
});

#[derive(Clone)]
pub struct Channel {
    pub uuid: String,
    pub remote_address: String,
    pub issuer: String,
    pub key: Option<String>,
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
        let mut channels_by_issuer = CHANNEL_BY_ISSUER.lock().await;
        if let Some(existing_channel) = channels_by_issuer.get(issuer) {
            return existing_channel.clone();
        }
        let channel = Arc::new(Channel {
            uuid: uuid.clone(),
            remote_address: remote_address.to_string(),
            issuer: issuer.to_string(),
            key,
            sessions: HashMap::new(),
        });
        channels_by_issuer.insert(issuer.to_string(), channel.clone());
        CHANNELS.lock().await.insert(uuid.clone(), channel.clone());
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