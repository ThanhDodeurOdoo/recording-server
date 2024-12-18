use log::info;
use uuid::Uuid;

use crate::config::RECORDING;
use crate::misc::schema_generated::ws_api::MediaSources;
use crate::models::ffmpeg::FFMPEG;

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
    pub fn start_recording(&self, media_sources: MediaSources) {
        self.start_fragment(media_sources);
    }
    pub fn stop_recording(&self) {
        // stop recording
    }
    fn start_fragment(&self, media_sources: MediaSources) {
        info!("fragment starting for {} audio", media_sources.audio().len());
        let ffmpeg = FFMPEG::new(format!(
            "{}/{}.{}",
            RECORDING.directory.display(),
            self.file_uuid,
            RECORDING.file_type
        ));
        info!("ffmpeg created at {}", ffmpeg.file_path);
    }
}
