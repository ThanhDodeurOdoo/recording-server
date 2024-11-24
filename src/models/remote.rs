use std::collections::HashMap;
use std::sync::{Arc, LazyLock, Mutex};
use serde::Serialize;
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
    pub remote_address: String,
    channels: HashMap<String, Channel>, // if a channel is here, a recording is in progress
}

impl Remote {
    pub fn new(remote_address: String) -> Remote {
        Remote {
            remote_address,
            channels: HashMap::new(),
        }
    }
    pub fn get_stats(&self) -> RemoteStats {
        RemoteStats {
            uuid: self.remote_address.clone(),
            channel_count: self.channels.len(),
        }
    }
    pub fn start_recording(&self, channel_uuid: &str) {
        // check if channel exists, if not create it and start recording, otherwise get the already existing channel and get the url where the recording is available (for streaming or download)
        // this should also generate a new uuid for the recording as one channel may have past different recordings that are temporarily stored even when the channel is removed.
    }
    pub fn stop_recording(&self, channel_uuid: &str) {
        // check if channel exists, if it does stop the recording, and return the uuid of the recording
        // this function will be called in the http service, which will store the recording uuid for future reference and routing to download.
    }
}
