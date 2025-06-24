use crate::misc::schema_generated::ws_api;
use crate::models::{recorder::Recorder, transcriptor::Transcriptor};
use actix_ws::{Message, MessageStream, Session};
use futures_util::StreamExt;
use log::{info, warn};
use std::collections::HashMap;

pub struct Remote {
    pub remote_address: String,
    session: Session,
    msg_stream: MessageStream,
    recorders: HashMap<String, Recorder>,
    transcriptors: HashMap<String, Transcriptor>,
}

impl Remote {
    pub fn new(remote_address: String, session: Session, msg_stream: MessageStream) -> Remote {
        Remote { 
            remote_address, 
            recorders: HashMap::new(), 
            transcriptors: HashMap::new(), 
            session,
            msg_stream,
        }
    }
    
    pub async fn listen(&mut self) {
        while let Some(Ok(message)) = self.msg_stream.next().await {
            match message {
                Message::Ping(ping) => {
                    if self.session.pong(&ping).await.is_err() {
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
                Message::Close(_) => {
                    break;
                }
                _ => {}
            }
        }

        let _ = self.session.clone().close(None).await;
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
    
    pub async fn send_text(&mut self, text: String) -> Result<(), actix_ws::Closed> {
        self.session.text(text).await
    }
    
    pub async fn send_binary(&mut self, data: Vec<u8>) -> Result<(), actix_ws::Closed> {
        self.session.binary(data).await
    }
}
