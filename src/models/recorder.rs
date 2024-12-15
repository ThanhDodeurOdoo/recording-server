use crate::misc::schema_generated::ws_api::MediaSource;
use uuid::Uuid;

#[derive(Clone)]
pub struct Recorder {
    pub uuid: String,
    pub remote_address: String,
    temp_files: Vec<String>,
    file_uuid: String,
}

impl Recorder {
    pub fn new(uuid: String, remote_address: String) -> Recorder {
        Recorder {
            uuid,
            remote_address,
            temp_files: Vec::new(),
            file_uuid: Uuid::new_v4().to_string(),
        }
    }
    pub fn start_recording(&self, audio_sources: Vec<MediaSource>, camera_sources: Vec<MediaSource>, screen_sources: Vec<MediaSource>) {
        self.start_fragment(audio_sources, camera_sources, screen_sources);
    }
    pub fn stop_recording(&self) {
        // stop recording
    }
    fn start_fragment(&self, audio_sources: Vec<MediaSource>, camera_sources: Vec<MediaSource>, screen_sources: Vec<MediaSource>) {
        // start fragment
    }
}
