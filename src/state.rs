use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::models::channel::ChannelMap;

pub static CHANNELS: Lazy<ChannelMap> = Lazy::new(|| {
    Arc::new(Mutex::new(HashMap::new()))
});
