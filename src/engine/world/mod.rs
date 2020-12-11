pub mod tile;
use std::collections::{hash_map, HashMap};

use self::tile::Tile;

pub struct World {
    pub tiles: HashMap<(u16, u16), Tile>,
}

impl World {
    pub fn new() -> Self {
        let tiles = HashMap::<(u16, u16), Tile>::new();
        Self { tiles }
    }

    pub fn insert_mut(
        &mut self,
        block: Tile,
    ) {
        match self.tiles.entry(block.coords()) {
            hash_map::Entry::Occupied(e) => *e.into_mut() += block,
            hash_map::Entry::Vacant(e) => {
                e.insert(block);
            }
        }
    }

    pub fn insert(
        &mut self,
        block: Tile,
    ) {
        self.tiles.insert(block.coords(), block);
    }

    pub fn test(&mut self) {
        for i in 0..10 {
            self.insert(Tile::new(u16::MAX / 2, u16::MAX / 2 + i).rand())
        }
    }

    pub fn update(&mut self) {}

    pub fn end(&mut self) { self.tiles = HashMap::new(); }
}
