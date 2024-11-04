use crate::models::channel::Channel;

#[derive(Clone)]
pub struct Recorder {
    pub channel: Channel
}

impl Recorder {
    pub fn new(channel: Channel) -> Recorder {
        Recorder { channel }
    }
    pub fn start_recording() {}
    pub fn pause_recording() {}
    pub fn stop_recording() {}
}
