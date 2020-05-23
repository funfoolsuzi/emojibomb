use super::*;

#[derive(Clone)]
pub enum Envelope {
    ShutDown,
    Register(std::sync::mpsc::SyncSender<std::sync::Arc<Envelope>>),
    SlotReserved(player::SlotReservedMsg),
    Confirm(Box<ConfirmMsg>),
    PlayerDelete(player::DeleteMsg),
    PlayerCreated(Box<player::CreatedMsg>),
    PlayerAdd(Box<player::AttachMsg>),
    PlayerMove(Box<player::MoveMsg>),
    MapCreate(Box<map::CreateMsg>),
}

impl Envelope {
    pub fn read_from(reader: &mut dyn std::io::Read, mtype: MsgType) -> std::io::Result<Self> {
        match mtype {
            MsgType::ShutDown => 
                Ok(Self::ShutDown),
            MsgType::SlotReserved =>
                Ok(Self::SlotReserved(player::SlotReservedMsg::read_from(reader)?)),
            MsgType::Confirm =>
                Ok(Self::Confirm(Box::new(ConfirmMsg::read_from(reader)?))),
            MsgType::PlayerAdd =>
                Ok(Self::PlayerAdd(Box::new(player::AttachMsg::read_from(reader)?))),
            MsgType::PlayerMove =>
                Ok(Self::PlayerMove(Box::new(player::MoveMsg::read_from(reader)?))),
            MsgType::PlayerCreated =>
                Ok(Self::PlayerCreated(Box::new(player::CreatedMsg::read_from(reader)?))),
            MsgType::PlayerDelete =>
                Ok(Self::PlayerDelete(player::DeleteMsg::read_from(reader)?)),
            MsgType::MapCreate =>
                Ok(Self::MapCreate(Box::new(map::CreateMsg::read_from(reader)?))),
            MsgType::Empty =>
                Err(std::io::Error::new(std::io::ErrorKind::Other, "Empty MsgType can't be read_from'd")),
        }
    }
    pub fn write_to(&self, writer: &mut dyn std::io::Write) -> std::io::Result<()> {
        match self {
            Self::Confirm(m) => (*m).write_to(writer),
            Self::PlayerAdd(m) => (*m).write_to(writer),
            Self::PlayerCreated(m) => (*m).write_to(writer),
            Self::PlayerMove(m) => (*m).write_to(writer),
            Self::MapCreate(m) => (*m).write_to(writer),
            Self::SlotReserved(m) => m.write_to(writer),
            Self::PlayerDelete(m) => m.write_to(writer), 
            Self::ShutDown | Self::Register(_)
            => Err(std::io::Error::new(std::io::ErrorKind::Other, "This type of Envelope can't be write_to'd")),
        }
    }
    pub fn msg_type(&self) -> MsgType {
        match self {
            Self::ShutDown => MsgType::ShutDown,
            Self::Confirm(_) => MsgType::Confirm,
            Self::SlotReserved(_) => MsgType::SlotReserved,
            Self::PlayerAdd(_) => MsgType::PlayerAdd,
            Self::PlayerMove(_) => MsgType::PlayerMove,
            Self::PlayerCreated(_) => MsgType::PlayerCreated,
            Self::PlayerDelete(_) => MsgType::PlayerDelete,
            Self::MapCreate(_) => MsgType::MapCreate,
            Self::Register(_) => MsgType::Empty,
        }
    }
}