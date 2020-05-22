use super::*;

#[derive(WriteTo, ReadFrom, Clone)]
pub struct CreatedMsg {
    name: [u8; 21],
    life: u32,
    coord: (u16, u16),
    id: u8,
}

impl From<Player> for CreatedMsg {
    fn from(player: Player) -> Self {
        let mut name_buf = [0u8; 21];
        let name = player.name.as_bytes();
        for i in 0..std::cmp::min(21, name.len()) {
            name_buf[i] = name[i]
        }
        Self {
            name: name_buf,
            life: player.life,
            coord: player.coord,
            id: player.id
        }
    }
}

impl Into<Player> for CreatedMsg {
    fn into(self) -> Player {
        Player::new(&self.name, self.id, self.coord, self.life)
    }
}