use std::collections::HashMap;
use futures_util::StreamExt;
use axum::extract::ws::{Message, WebSocket};
use log::{info};
use crate::models::recorder::Recorder;
use crate::misc::flatbuffer_types;

pub struct Remote {
    pub remote_address: String,
    socket: WebSocket,
    recorders: HashMap<String, Recorder>
}

impl Remote {
    pub fn new(remote_address: String, socket: WebSocket) -> Remote {
        Remote {
            remote_address,
            recorders: HashMap::new(),
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
                    let data = flatbuffers::root::<flatbuffer_types::recording::Message>(&bin).unwrap();
                    match data.type_() {
                        "recording" => {
                            // could directly receive ports to listen to (4 video ports max) + audio ports.
                            let recording = data.content_as_recording_payload().unwrap();
                            let channel_uuid = recording.channel_uuid();
                            self.start_recording(channel_uuid);
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
    fn start_recording(&self, channel_uuid: &str) {
        // check if channel exists, if not create it and start recording, otherwise get the already existing channel and get the url where the recording is available (for streaming or download)
        // this should also generate a new uuid for the recording as one channel may have past different recordings that are temporarily stored even when the channel is removed.
    }
    fn stop_recording(&self, channel_uuid: &str) {
        // check if channel exists, if it does stop the recording, and return the uuid of the recording
        // this function will be called in the http service, which will store the recording uuid for future reference and routing to download.
    }
    pub fn cleanup(&self) {
    }
}
