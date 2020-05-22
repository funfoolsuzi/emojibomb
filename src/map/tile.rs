#[derive(Copy, Clone, PartialEq, Debug)]
pub enum LandScape {
    Grass,
    Water,
    Woods,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Tile {
    landscape: LandScape,
}

impl Tile {
    pub fn landscape(&self) -> &LandScape {
        &self.landscape
    }
}

impl Default for Tile {
    fn default() -> Self {
        Self {
            landscape: LandScape::Grass
        }
    }
}