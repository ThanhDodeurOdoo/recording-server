use std::collections::HashMap;
use crate::models::channel::Channel;
#[derive(Clone)]
pub struct Sfu {
    pub id: String, // or u16? should probably be the remote address (+ some identifier?)
    pub key: String, // used to auth sfus with recorders, although entry points could be locked by nginx
    pub channels: HashMap<String, Channel>
}
