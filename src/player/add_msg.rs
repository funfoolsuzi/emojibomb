use super::*;

#[derive(WriteTo, ReadFrom, Clone)]
pub struct AttachMsg {
    pub client_id: u8,
    pub name: [u8; 21],
}

impl AttachMsg {
    pub fn new(client_id: u8, name_bytes: &[u8]) -> Self {
        let mut name = [0u8; 21];
        for i in 0..std::cmp::min(name_bytes.len(), 21) {
            name[i] = name_bytes[i]
        }
        Self {
            client_id,
            name: name,
        }
    }
}

#[derive(WriteTo, ReadFrom, Clone, Copy)]
pub struct SlotReservedMsg(pub Option<u8>);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_player_add_msg() {
        let msgvec: Vec<u8> = vec!(0x0 , 0x41, 0x6c, 0x66, 0x72, 0x65, 0x64, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0);
        let mut buf_reader = std::io::BufReader::<&[u8]>::new(&msgvec);
        let msg: AttachMsg = AttachMsg::read_from(&mut buf_reader).unwrap();
        let p = msg.name;
        assert_eq!(p[0], 0x41);
        assert_eq!(p[5], 0x64);
        assert_eq!(p[6], 0x0);
    }
}