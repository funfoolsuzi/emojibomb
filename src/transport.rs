use crate::msg::{ MsgType};
use emojibomb_derive::{WriteTo, ReadFrom};

pub const CLIENT_MAGIC_NUM: u8 = 0x89;
pub const SERVER_MAGIC_NUM: u8 = 0x64;

#[derive(PartialEq, Eq, Hash, Clone, Copy, WriteTo, ReadFrom)]
pub struct ClientHeader {
    magic: u8,
    mtype: MsgType,
    client_id: u8,
    msg_id: u32,
}

impl ClientHeader {
    pub fn new(mtype: MsgType, client_id: u8, msg_id: u32) -> Self {
        Self {
            magic: CLIENT_MAGIC_NUM,
            mtype,
            client_id,
            msg_id
        }
    }
    pub fn mtype(&self) -> MsgType {self.mtype}
    pub fn player_id(&self) -> u8 {self.client_id}
    pub fn msg_id(&self) -> u32 {self.msg_id}
}

impl Default for ClientHeader {
    fn default() -> Self {
        Self::new(MsgType::Empty, 0, 0)
    }
}

#[derive(WriteTo, ReadFrom)]
pub struct ServerHeader {
    magic: u8,
    mtype: MsgType,
}

impl ServerHeader {
    pub fn new(mtype: MsgType) -> Self {
        Self {
            magic: CLIENT_MAGIC_NUM,
            mtype,
        }
    }
    pub fn mtype(&self) -> MsgType {
        self.mtype
    }
    pub fn valid(&self) -> bool {
        self.magic == CLIENT_MAGIC_NUM
    }
}
impl Default for ServerHeader {
    fn default() -> Self {
        Self::new(MsgType::Empty)
    }
}

#[cfg(test)]
mod tests {

}