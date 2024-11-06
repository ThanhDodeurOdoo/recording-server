use std::collections::HashMap;
use std::sync::{Arc, LazyLock};
use serde::Serialize;
use tokio::sync::Mutex;
use crate::models::channel::Channel;

pub static REMOTES: LazyLock<Arc<Mutex<HashMap<String, Arc<Remote>>>>> = LazyLock::new(|| {
    Arc::new(Mutex::new(HashMap::new()))
});

#[derive(Serialize)]
pub struct RemoteStats {
    uuid: String,
    channel_count: usize,
}

#[derive(Clone)]
pub struct Remote {
    remote_address: String,
    channels: HashMap<String, Channel>,
    websocket: String, // should be a ws type, taking ownership of the socket once auth is done
}

impl Remote {
    pub fn new(remote_address: String) -> Remote {
        Remote {
            remote_address,
            channels: HashMap::new(),
            websocket: String::from("TODO"),
        }
    }
    pub fn get_stats(&self) -> RemoteStats {
        RemoteStats {
            uuid: self.remote_address.clone(),
            channel_count: self.channels.len(),
        }
    }
    pub fn add_channel(&mut self, channel: Channel) {
        // some logic that creates channel,
    }
}
