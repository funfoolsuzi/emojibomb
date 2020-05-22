use super::*;

#[derive(Clone)]
pub enum Map {
    M20x30([[tile::Tile; 30]; 20]),
    M25x40([[tile::Tile; 40]; 25]),
}

impl Default for Map {
    fn default() -> Self {
        Map::M20x30([[tile::Tile::default(); 30]; 20])
    }
}

macro_rules! impl_get {
    ( $( ($case: path, $h: expr, $w: expr) ),* ) => {
        pub fn get(&self, x: usize, y: usize) -> Option<&Tile> {
            match self {
                $(
                    $case(m) => {
                        if x >= $h || y >= $w {
                            return None;
                        }
                        return Some(&m[x][y]);
                    }
                ),*
            }
        }
        pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut Tile> {
            match self {
                $(
                    $case(m) => {
                        if x >= $h || y >= $w {
                            return None;
                        }
                        return Some(&mut m[x][y]);
                    }
                ),*
            }
        }
    };
}

impl Map {
    impl_get!(
        (Self::M20x30, 20, 30),
        (Self::M25x40, 25, 40)
    );
    pub fn get_size(&self) -> (usize, usize) {
        match self {
            Self::M20x30(_) => (20, 30),
            Self::M25x40(_) => (25, 40),
        }
    }
    pub fn get_size_u16(&self) -> (u16, u16) {
        let s = self.get_size();
        (s.0 as u16, s.1 as u16)
    }
    pub fn get_iter(&self) -> MapIter {
        MapIter::new(self)
    }
}

pub struct MapIter<'a> {
    r: usize,
    c: usize,
    map: &'a Map,
}

impl<'a> MapIter<'a> {
    pub fn new(map: &'a Map) -> Self {
        Self{
            r: 0, c: 0, map,
        }
    }
}

impl<'a> Iterator for MapIter<'a> {
    type Item = (usize, usize, &'a Tile);
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(t) = self.map.get(self.r, self.c) {
            self.c += 1;
            return Some((self.r, self.c-1, t));
        } else {
            if let Some(t) = self.map.get(self.r + 1, 0) {
                self.r += 1;
                self.c = 1;
                return Some((self.r, self.c-1, t));
            } else {
                return None
            }
        }

    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tile_access() {
        let map = Map::default();
        assert_eq!(map.get(40,40), None);
        assert_eq!(map.get(0, 0).unwrap().clone(), Tile::default());
    }
}