
use emojibomb_derive::{WriteTo, ReadFrom};

#[derive(Clone, WriteTo, ReadFrom)]
pub struct ConfirmMsg {
    msg_id: u32,
    status: StatusCode,
}

impl ConfirmMsg {
    pub fn new(msg_id: u32, status: StatusCode) -> Self {
        Self {msg_id, status}
    }
    pub fn msg_id(&self) -> u32 {
        self.msg_id
    }
    pub fn valid(&self) -> bool {
        self.status == StatusCode::OK
    }
}

#[repr(u8)]
#[derive(PartialEq, Clone, Copy)]
pub enum StatusCode {
    OK = 0,
    Unparsable = 1,
    UnhandlableType = 2,
    InvalidMove = 3,
    FailToAddPlayer = 4,
    Break = 255
}

impl Into<u8> for StatusCode {
    fn into(self) -> u8 {
        self as u8
    }
}