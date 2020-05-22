use crate::{map::Map};
use emojibomb_derive::{WriteTo, ReadFrom};

#[derive(WriteTo, ReadFrom, Clone)]
pub struct MapCreateMsg(Map);

impl MapCreateMsg {
    pub fn new(m: Map) -> Self {
        Self(m)
    }
}

impl Into<Map> for MapCreateMsg {
    fn into(self) -> Map {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_new_map_msg() {
        let m = MapCreateMsg(Map::default());
        let a_tile = m.0.get(5,5).unwrap();
        assert_eq!(a_tile.landscape(), crate::map::Tile::default().landscape())
    }
}