use log::info;
use uuid::Uuid;

use crate::config::RECORDING;
use crate::misc::schema_generated::ws_api::MediaSources;
use crate::models::ffmpeg::FFMPEG;

pub struct Recorder {
    pub uuid: String,
    pub remote_address: String,
    temp_files: Vec<String>,
    file_uuid: String,
    ffmpeg: Option<FFMPEG>,
}

impl Recorder {
    pub fn new(uuid: String, remote_address: String) -> Recorder {
        Recorder {
            uuid,
            remote_address,
            temp_files: Vec::new(),
            file_uuid: Uuid::new_v4().to_string(),
            ffmpeg: None,
        }
    }
    pub async fn start_recording(&mut self, media_sources: MediaSources<'_>) {
        self.start_fragment(media_sources).await;
    }
    pub fn stop_recording(&self) {
        // concat the files of self.temp_files if more than 1. return the merge or the 1 file, then clear self.temp_files
    }
    async fn start_fragment(&mut self, media_sources: MediaSources<'_>) {
        info!("fragment starting for {} audio", media_sources.audio().len());
        let mut ffmpeg = FFMPEG::new(format!(
            "{}/{}_{}.{}",
            RECORDING.directory.display(),
            self.file_uuid,
            self.temp_files.len(),
            RECORDING.file_type
        ));
        match ffmpeg.merge(media_sources).await {
            Ok(file_path) => {
                self.temp_files.push(file_path.to_string());
                info!("fragment finished for {}", file_path);
            }
            Err(e) => {
                eprintln!("Failed to merge media sources: {e}",);
                return;
            }
        }
        // killing old process as soon as the new one takes over to minimize swap stuttering
        if let Some(ffmpeg) = &mut self.ffmpeg {
            ffmpeg.kill().await;
        }
        self.ffmpeg = Some(ffmpeg);
    }
}
