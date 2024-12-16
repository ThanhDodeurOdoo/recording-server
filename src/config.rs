#![allow(clippy::module_name_repetitions)]
use std::env;
use std::env::temp_dir;
use std::path::PathBuf;
use std::sync::LazyLock;
use std::thread;
use std::time::Duration;

pub static AUTH_KEY: LazyLock<String> =
    LazyLock::new(|| env::var("AUTH_KEY").expect("AUTH_KEY environment variable not set"));
pub static PUBLIC_IP: LazyLock<String> =
    LazyLock::new(|| env::var("PUBLIC_IP").expect("PUBLIC_IP environment variable not set"));
pub static RTC_INTERFACE: LazyLock<String> = LazyLock::new(|| {
    env::var("RTC_INTERFACE").expect("RTC_INTERFACE environment variable not set")
});
pub static PROXY: LazyLock<bool> = LazyLock::new(|| env::var("PROXY").is_ok());
pub static HTTP_INTERFACE: LazyLock<String> = LazyLock::new(|| {
    env::var("HTTP_INTERFACE").expect("HTTP_INTERFACE environment variable not set")
});
pub static PORT: LazyLock<u16> = LazyLock::new(|| {
    env::var("PORT").ok().and_then(|port_str| port_str.parse::<u16>().ok()).unwrap_or(8070)
});
pub static NUM_WORKERS: LazyLock<u16> = LazyLock::new(|| {
    // could be usize type?
    let env_workers =
        env::var("NUM_WORKERS").ok().and_then(|num| num.parse::<u16>().ok()).unwrap_or(u16::MAX);

    let available_workers = thread::available_parallelism().map(|n| n.get() as u16).unwrap_or(1);

    env_workers.min(available_workers)
});

pub struct RecordingConfig {
    pub directory: PathBuf,
    pub enabled: bool,
    pub max_duration: Duration,
    pub file_ttl: Duration,
    pub file_type: &'static str,
    pub video_codec: &'static str,
    pub audio_codec: &'static str,
    pub audio_limit: u8,
    pub camera_limit: u8,
    pub screen_limit: u8,
}
pub static RECORDING: LazyLock<RecordingConfig> = LazyLock::new(|| {
    RecordingConfig {
        directory: temp_dir().join("recordings"),
        enabled: env::var("RECORDING").is_ok(),
        max_duration: Duration::from_millis(1000 * 60 * 60), // 1 hour
        file_ttl: Duration::from_millis(1000 * 60 * 60 * 24), // 24 hours
        file_type: "mp4",
        video_codec: "libx264",
        audio_codec: "aac",
        audio_limit: 20,
        camera_limit: 4,
        screen_limit: 1,
    }
});

// should be called early to make sure that these variables are available at runtime.
pub fn initialize_required_globals() {
    LazyLock::force(&AUTH_KEY);
    LazyLock::force(&PUBLIC_IP);
    LazyLock::force(&RTC_INTERFACE);
    LazyLock::force(&HTTP_INTERFACE);
}

// TODO continue reimplementing https://github.com/odoo/sfu/blob/master/src/config.js
