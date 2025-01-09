use crate::misc::schema_generated::ws_api;
use crate::models::{recorder::Recorder, transcriptor::Transcriptor};
use axum::extract::ws::{Message, WebSocket};
use futures_util::StreamExt;
use log::{info, warn};
use std::collections::HashMap;

pub struct Remote {
    pub remote_address: String,
    socket: WebSocket,
    recorders: HashMap<String, Recorder>,
    transcriptors: HashMap<String, Transcriptor>,
}

impl Remote {
    pub fn new(remote_address: String, socket: WebSocket) -> Remote {
        Remote { remote_address, recorders: HashMap::new(), transcriptors: HashMap::new(), socket }
    }
    pub async fn listen(&mut self) {
        // .next() uses futures_util::StreamExt
        while let Some(Ok(message)) = self.socket.next().await {
            match message {
                Message::Ping(ping) => {
                    if self.socket.send(Message::Pong(ping)).await.is_err() {
                        return;
                    }
                }
                Message::Text(text) => info!("{} ws message/text: {}", self.remote_address, text),
                Message::Binary(bin) => {
                    let Some(data) = flatbuffers::root::<ws_api::Message>(&bin).ok() else {
                        warn!("Failed to parse message, message are supposed to be flatbuffers matching the schema of the websocket API");
                        return;
                    };
                    match data.action() {
                        ws_api::Action::start_recording => {
                            let Some(recording) = data.content_as_recording_payload() else {
                                warn!("Failed to get recording payload");
                                return;
                            };
                            let channel_uuid = data.channel_uuid();
                            self.start_recording(channel_uuid, recording.media_sources()).await;
                            info!(
                                "{} recording requested for: {}",
                                self.remote_address, channel_uuid
                            );
                        }
                        ws_api::Action::start_transcript => {
                            // let transcription = data.content_as_transcription_payload().unwrap();
                            let channel_uuid = data.channel_uuid();
                            info!(
                                "{} transcription requested for: {}",
                                self.remote_address, channel_uuid
                            );
                        }
                        _ => info!("{} unknown or missing message type", self.remote_address),
                    }
                }
                _ => break,
            }
        }
        self.cleanup();
    }
    async fn start_recording(
        &mut self,
        channel_uuid: &str,
        media_sources: ws_api::MediaSources<'_>,
    ) {
        let recorder = self.recorders.entry(channel_uuid.to_string()).or_insert_with(|| {
            Recorder::new(channel_uuid.to_string(), self.remote_address.clone())
        });
        recorder.start_recording(media_sources).await;
    }
    fn stop_recording(&mut self, channel_uuid: &str) {
        if let Some(recorder) = self.recorders.get(channel_uuid) {
            recorder.stop_recording();
            self.recorders.remove(channel_uuid);
        } else {
            warn!("Could not stop Recording: recorder not found for channel {}", channel_uuid);
        }
    }
    pub fn cleanup(&self) {}
}
