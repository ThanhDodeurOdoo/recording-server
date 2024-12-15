use std::collections::HashMap;
use futures_util::StreamExt;
use axum::extract::ws::{Message, WebSocket};
use log::{info};
use crate::models::{recorder::Recorder, transcriptor::Transcriptor};
use crate::misc::flatbuffer_types;

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
                    // Parse the flatbuffer message
                    let data = flatbuffers::root::<flatbuffer_types::sfu::Message>(&bin).unwrap();
                    match data.type_() {
                        "recording" => {
                            // could directly receive ports to listen to (4 video ports max) + audio ports.
                            let recording = data.content_as_recording_payload().unwrap();
                            let channel_uuid = recording.channel_uuid();
                            let audio_sources: Vec<_> = recording.audio_sources().iter().collect();
                            let camera_sources: Vec<_> = recording.camera_sources().iter().collect();
                            let screen_sources: Vec<_> = recording.screen_sources().iter().collect();
                            self.start_recording(channel_uuid, audio_sources, camera_sources, screen_sources);
                            info!("{} recording started", self.remote_address);
                        }
                        "command" => {
                            let command = data.content_as_command_payload().unwrap();
                            let channel_uuid = command.channel_uuid();
                            let name = command.name();
                            info!("{} command received: {}", self.remote_address, name);
                        }
                        _ => info!("{} unknown or missing message type", self.remote_address),
                    }

                }
                _ => break,
            }
        }
        self.cleanup();
    }
    fn start_recording(&mut self, channel_uuid: &str, audio_sources: Vec<flatbuffer_types::sfu::MediaSource>, camera_sources: Vec<flatbuffer_types::sfu::MediaSource>, screen_sources: Vec<flatbuffer_types::sfu::MediaSource>) {
        let recorder = self.recorders.entry(channel_uuid.to_string()).or_insert_with(|| Recorder::new(channel_uuid.to_string(), self.remote_address.clone()));
        recorder.start_recording(audio_sources, camera_sources, screen_sources);
    }
    fn stop_recording(&mut self, channel_uuid: &str) {
        let mut recorder = self.recorders.get(channel_uuid).unwrap();
        recorder.stop_recording();
        self.recorders.remove(channel_uuid);
    }
    pub fn cleanup(&self) {
    }
}
