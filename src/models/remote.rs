use std::collections::HashMap;
use futures_util::StreamExt;
use axum::extract::ws::{Message, WebSocket};
use log::{info};
use crate::models::{recorder::Recorder, transcriptor::Transcriptor};
use crate::misc::schema_generated::ws_api;

pub struct Remote {
    pub remote_address: String,
    socket: WebSocket,
    recorders: HashMap<String, Recorder>,
    transcriptors : HashMap<String, Transcriptor>
}

impl Remote {
    pub fn new(remote_address: String, socket: WebSocket) -> Remote {
        Remote {
            remote_address,
            recorders: HashMap::new(),
            transcriptors: HashMap::new(),
            socket,
        }
    }
    pub async fn listen(&mut self) {
        while let Some(Ok(msg)) = self.socket.next().await {
            match msg {
                Message::Ping(ping) => {
                    if self.socket.send(Message::Pong(ping)).await.is_err() {
                        return;
                    }
                }
                Message::Text(text) => info!("{} ws message/text: {}", self.remote_address, text),
                Message::Binary(bin) => {
                    let data = flatbuffers::root::<ws_api::Message>(&bin).unwrap();
                    match data.action() {
                        "start-recording" => {
                            let recording = data.content_as_recording_payload().unwrap();
                            let channel_uuid = data.channel_uuid();
                            self.start_recording(channel_uuid, recording.media_sources());
                            info!("{} recording requested for: {}", self.remote_address, channel_uuid);
                        }
                        "start-transcript" => {
                            // let transcription = data.content_as_transcription_payload().unwrap();
                            let channel_uuid = data.channel_uuid();
                            info!("{} transcription requested for: {}", self.remote_address, channel_uuid);
                        }
                        _ => info!("{} unknown or missing message type", self.remote_address),
                    }

                }
                _ => break,
            }
        }
        self.cleanup();
    }
    fn start_recording(&mut self, channel_uuid: &str, media_sources: ws_api::MediaSources) {
        let recorder = self.recorders.entry(channel_uuid.to_string()).or_insert_with(|| Recorder::new(channel_uuid.to_string(), self.remote_address.clone()));
        recorder.start_recording(media_sources);
    }
    fn stop_recording(&mut self, channel_uuid: &str) {
        let recorder = self.recorders.get(channel_uuid).unwrap();
        recorder.stop_recording();
        self.recorders.remove(channel_uuid);
    }
    pub fn cleanup(&self) {
    }
}
