use std::sync::Arc;
use crate::map::Map;
use crate::msg::{Envelope, MapCreateMsg};
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
    pub fn act(&self, players: &mut Store, map: &mut Map) {
        let mut new_player = player::Player::new(
            &self.name, self.client_id,
            (0, 0), 100
        );
        let map_size = map.get_size();
        players.attach_player(&mut new_player, (map_size.0 as u16, map_size.1 as u16));
        let env = Envelope::PlayerCreated(Box::new(new_player.into()));
        let ptr = Arc::new(env);
        let mut existing_player_msgs: Vec<Arc<Envelope>> = vec!();
        for entry in players.iter() {
            if let Some(player) = &entry.player {
                if player.id != self.client_id {
                    existing_player_msgs.push(Arc::new(Envelope::PlayerCreated(Box::new(player.clone().into()))));
                }
                entry.sender.send(ptr.clone()).unwrap();                
            }
        }
        let to_new_client = players.get_sender(self.client_id).unwrap();
        to_new_client.send(
            Arc::new(Envelope::MapCreate(Box::new(MapCreateMsg::new(map.clone()))))
        ).unwrap();
        for m in existing_player_msgs {
            to_new_client.send(m).unwrap();
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
        let msgvec: Vec<u8> = vec!(0x41, 0x6c, 0x66, 0x72, 0x65, 0x64, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0);
        let mut buf_reader = std::io::BufReader::<&[u8]>::new(&msgvec);
        let msg: AttachMsg = AttachMsg::read_from(&mut buf_reader).unwrap();
        let p = msg.name;
        assert_eq!(p[0], 0x41);
        assert_eq!(p[5], 0x64);
        assert_eq!(p[6], 0x0);
    }
}