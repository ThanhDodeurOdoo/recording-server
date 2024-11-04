use std::collections::HashMap;
use std::sync::{mpsc::{channel, Receiver, Sender}, Arc, Mutex};

pub struct Channel {
    pub rc: Option<Receiver<String>>,
    pub tx: Sender<String>,
}

pub struct Bus {
    pub channels: HashMap<String, Arc<Mutex<Channel>>>,
}

impl Bus {
    pub fn new() -> Self {
        Bus {
            channels: HashMap::new(),
        }
    }

    pub fn register(&mut self, name: String) {
        let (tx, rc) = channel();
        let channel = Channel { rc: Some(rc), tx };
        self.channels.insert(name, Arc::new(Mutex::new(channel)));
    }

    pub fn send(&self, target: &str, message: String) {
        if let Some(channel) = self.channels.get(target) {
            if let Ok(channel) = channel.lock() {
                channel.tx.send(message).expect("Failed to send message");
            }
        }
    }

    // Allows a service to take ownership of its receiver
    pub fn take_receiver(&self, name: &str) -> Option<Receiver<String>> {
        self.channels.get(name).and_then(|channel| {
            let mut channel = channel.lock().ok()?;
            channel.rc.take()
        })
    }
}