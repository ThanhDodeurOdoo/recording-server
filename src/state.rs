use std::sync::{{ Arc, LazyLock }};
use std::collections::HashMap;
use tokio::sync::Mutex;
use crate::models::channel::ChannelMap;

pub static CHANNELS: LazyLock<ChannelMap> = LazyLock::new(|| {
    Arc::new(Mutex::new(HashMap::new()))
});
