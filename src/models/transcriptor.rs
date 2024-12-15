use crate::misc::flatbuffer_types;
use uuid::Uuid;

#[derive(Clone)]
pub struct Transcriptor {
    pub uuid: String,
    pub remote_address: String,
    file_uuid: String,
}

impl Transcriptor {
    pub fn new(uuid: String, remote_address: String) -> Transcriptor {
        Transcriptor {
            uuid,
            remote_address,
            file_uuid: Uuid::new_v4().to_string(),
        }
    }
    pub fn start_transcript(&self, audio_sources: Vec<flatbuffer_types::sfu::MediaSource>) {

    }
    pub fn stop_transcript(&self) {

    }
}
