pub mod http;
pub mod rtc;
pub mod ws;

use std::sync::{Arc, Mutex};
use crate::models::bus::Bus;

pub trait Service: Send + Sync where Self: 'static {
    fn name(&self) -> &str;
    fn bus(&self) -> Arc<Mutex<Bus>>;

    // messages should be serialized and deserialized with JSON (or flat buffers?)
    fn send_message(&self, target: &str, message: &str) {
        self.bus().lock().unwrap().send(target, message.to_string());
    }

    fn handle_message(&self, message: String);

    fn start(self: Arc<Self>) -> impl std::future::Future<Output = ()> + Send {async {
        self.bus().lock().unwrap().register(self.name().to_string());
        self.start_listening();
    }}

    fn start_listening(self: Arc<Self>) {
        if let Some(receiver) = self.bus().lock().unwrap().take_receiver(self.name()) {
            let service_name = self.name().to_string();
            std::thread::spawn(move || {
                while let Ok(message) = receiver.recv() {
                    println!("[{}] Received: {}", service_name, message);
                    self.handle_message(message);
                }
                println!("[{}] Listener stopped", service_name);
            });
        } else {
            eprintln!("No receiver found for {}", self.name());
        }
    }
}
