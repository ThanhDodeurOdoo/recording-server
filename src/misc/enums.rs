#[repr(u16)]
pub enum WsCloseCode {
    Clean = 1000,
    Leaving = 1001,
    Error = 1011,
    // 4000-4999 range available for app specific use
    AuthenticationFailed = 4106,
    Timeout = 4107,
    Kicked = 4108,
    ChannelFull = 4109,
}
