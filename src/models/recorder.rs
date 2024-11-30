#[derive(Clone)]
pub struct Recorder {
    pub uuid: String,
    pub recording_uuid: String,
    pub remote_address: String,
    pub issuer: String,
    pub key: Option<String>,
    pub router: String,
}
