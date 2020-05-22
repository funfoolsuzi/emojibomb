#[derive(Clone)]
pub struct Player {
    pub name: String,
    pub life: u32,
    pub coord: (u16, u16),
    pub id: u8,
}

impl Player {
    pub fn new(name_buf: &[u8], id: u8, coord: (u16, u16), life: u32) -> Self {
        let mut n: String = String::with_capacity(name_buf.len());
        for c in name_buf {
            if *c == 0 {
                break
            }
            n.push((*c) as char)
        }
        Self {
            name: n,
            life: life,
            coord: coord,
            id: id,
        }
    }
    pub fn is_alive(&self) -> bool { self.life == 0 }
}
