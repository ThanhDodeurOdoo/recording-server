use once_cell::sync::Lazy;
use std::env;
use std::thread;

pub static AUTH_KEY: Lazy<String> = Lazy::new(|| {
    env::var("AUTH_KEY").expect("AUTH_KEY environment variable not set")
});
pub static PUBLIC_IP: Lazy<String> = Lazy::new(|| {
    env::var("PUBLIC_IP").expect("PUBLIC_IP environment variable not set")
});
pub static RTC_INTERFACE: Lazy<String> = Lazy::new(|| {
    env::var("RTC_INTERFACE").expect("RTC_INTERFACE environment variable not set")
});
pub static PROXY: Lazy<bool> = Lazy::new(|| env::var("PROXY").is_ok());
pub static HTTP_INTERFACE: Lazy<String> = Lazy::new(|| {
    env::var("HTTP_INTERFACE").expect("HTTP_INTERFACE environment variable not set")
});
pub static PORT: Lazy<u16> = Lazy::new(|| {
    env::var("PORT")
        .ok()
        .and_then(|port_str| port_str.parse::<u16>().ok())
        .unwrap_or(8070)
});
pub static NUM_WORKERS: Lazy<u16> = Lazy::new(|| {
    let env_workers = env::var("NUM_WORKERS")
        .ok()
        .and_then(|num| num.parse::<u16>().ok())
        .unwrap_or(u16::MAX);

    let available_workers = thread::available_parallelism()
        .map(|n| n.get() as u16)
        .unwrap_or(1);

    env_workers.min(available_workers)
});
pub static AUDIO_CODECS: Lazy<Option<Vec<String>>> = Lazy::new(|| {
    env::var("AUDIO_CODECS").ok().map(|codecs| {
        codecs.split(',')
            .map(|codec| codec.trim().to_string())
            .collect()
    })
});
pub static VIDEO_CODECS: Lazy<Option<Vec<String>>> = Lazy::new(|| {
    env::var("VIDEO_CODECS").ok().map(|codecs| {
        codecs.split(',')
            .map(|codec| codec.trim().to_string())
            .collect()
    })
});

// TODO continue reimplementing https://github.com/odoo/sfu/blob/master/src/config.js