use super::{tile::Tile, Point};
use array_macro::array;
use indexmap::Equivalent;
use std::hash::{Hash, Hasher};

#[derive(Eq, Clone, Debug)]
pub struct Chunk {
    pub pos:     Point<u16>,
    pub tiles:   [Tile; 1024],
    pub border:  [Option<Tile>; 128],
    pub changed: bool,
}

impl Default for Chunk {
    fn default() -> Self {
        let tiles = array![x=>Tile::new(&Point((x/32) as u8,((x as f64/32.).fract()*32. )as u8)); 1024];
        let border = array![None; 128];
        let changed = false;
        Self {
            pos: Point::default(),
            tiles,
            border,
            changed,
        }
    }
}
impl Chunk {
    pub fn on_screen(
        &self,
        camera: (u16, u16, u16, u16),
    ) -> bool {
        self.pos.0 >= camera.0.saturating_sub(camera.2) &&
            self.pos.1 >= camera.1.saturating_sub(camera.3) &&
            self.pos.0 <= camera.0.saturating_add(camera.2) &&
            self.pos.1 <= camera.1.saturating_add(camera.3)
    }
}

impl PartialEq for Chunk {
    fn eq(
        &self,
        other: &Self,
    ) -> bool {
        self.pos == other.pos
    }
}
impl Hash for Chunk {
    fn hash<H: Hasher>(
        &self,
        state: &mut H,
    ) {
        self.pos.hash(state);
    }
}

impl Equivalent<Point<u16>> for Chunk {
    fn equivalent(
        &self,
        key: &Point<u16>,
    ) -> bool {
        &self.pos == key
    }
}
impl Equivalent<Chunk> for Point<u16> {
    fn equivalent(
        &self,
        key: &Chunk,
    ) -> bool {
        self == &key.pos
    }
}
