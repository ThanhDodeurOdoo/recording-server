use std::collections::HashMap;
use futures_util::StreamExt;
use axum::extract::ws::{Message, WebSocket};
use serde::Serialize;
use crate::models::channel::Channel;
use crate::misc::flatbuffer_types;

#[derive(Serialize)]
pub struct RemoteStats {
    uuid: String,
    channel_count: usize,
}
pub struct Remote {
    pub remote_address: String,
    socket: WebSocket,
    channels: HashMap<String, Channel>, // if a channel is here, a recording is in progress
}

impl Remote {
    pub fn new(remote_address: String, socket: WebSocket) -> Remote {
        Remote {
            remote_address,
            channels: HashMap::new(),
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
                Message::Text(text) => println!("ws message/text: {text}"),
                Message::Binary(bin) => {
                    // Parse the flatbuffer message
                    let data = flatbuffers::root::<flatbuffer_types::recording::Message>(&bin).unwrap();
                    match data.type_() {
                        "recording" => {
                            let recording = data.content_as_recording_payload().unwrap();
                            let channel_uuid = recording.channel_uuid();
                            let capabilities = recording.capabilities();
                            let transport_port = recording.transport_port();
                            let transport_host = recording.transport_host();
                            self.start_recording(channel_uuid);
                        }
                        "command" => {
                            let command = data.content_as_command_payload().unwrap();
                            let channel_uuid = command.channel_uuid();
                            let name = command.name();
                        }
                        _ => println!("Unknown type"),
                    }

                }
                _ => break,
            }
        }
    }
    pub fn get_stats(&self) -> RemoteStats {
        RemoteStats {
            uuid: self.remote_address.clone(),
            channel_count: self.channels.len(),
        }
    }
    pub fn start_recording(&self, channel_uuid: &str) {
        // check if channel exists, if not create it and start recording, otherwise get the already existing channel and get the url where the recording is available (for streaming or download)
        // this should also generate a new uuid for the recording as one channel may have past different recordings that are temporarily stored even when the channel is removed.
    }
    pub fn stop_recording(&self, channel_uuid: &str) {
        // check if channel exists, if it does stop the recording, and return the uuid of the recording
        // this function will be called in the http service, which will store the recording uuid for future reference and routing to download.
    }
    pub fn cleanup(&self) {
    }
}
