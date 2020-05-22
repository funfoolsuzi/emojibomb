use super::*;

#[derive(WriteTo, ReadFrom, Clone)]
pub struct DeleteMsg {
    pub id: u8,
}