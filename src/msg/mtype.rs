use super::*;

#[repr(u16)]
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum MsgType {
    Empty = 0,
    ShutDown,
    Confirm,
    SlotReserved,
    PlayerAdd,
    PlayerMove,
    PlayerCreated,
    PlayerDelete,
    MapCreate,
}

impl MsgType {
    pub fn msg_size(&self) -> usize {
        match self {
            Self::ShutDown | Self::Empty => 0,
            Self::Confirm => std::mem::size_of::<super::ConfirmMsg>(),
            Self::SlotReserved => std::mem::size_of::<player::SlotReservedMsg>(),
            Self::PlayerAdd => std::mem::size_of::<player::AttachMsg>(),
            Self::PlayerMove => std::mem::size_of::<player::MoveMsg>(),
            Self::PlayerCreated => std::mem::size_of::<player::CreatedMsg>(),
            Self::PlayerDelete => std::mem::size_of::<player::DeleteMsg>(),
            Self::MapCreate => std::mem::size_of::<super::MapCreateMsg>(),
        }        
    }
}

#[cfg(test)]
mod tests {

}