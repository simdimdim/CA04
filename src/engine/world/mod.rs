pub mod tile;
use std::collections::{hash_map, HashMap, HashSet};

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

    pub fn remove(
        &mut self,
        block: &(u16, u16),
    ) {
        self.tiles.remove(block);
    }

    pub fn put(
        &mut self,
        block: (u16, u16),
    ) {
        self.tiles.insert(block, Tile::new(block.0, block.1).rand());
    }

    pub fn test(&mut self) {
        for i in 0..10 {
            self.insert(Tile::new(u16::MAX, i).rand());
        }
    }

    pub fn update(&mut self) {
        let mut tmp = HashSet::new();
        let mut newtiles = HashMap::new();
        for (k, v) in self.tiles.iter() {
            tmp.insert((k.0 - 1, k.1 - 1));
            tmp.insert((k.0, k.1 - 1));
            tmp.insert((k.0 + 1, k.1 - 1));
            tmp.insert((k.0 - 1, k.1));
            tmp.insert((k.0 + 1, k.1));
            tmp.insert((k.0 - 1, k.1 + 1));
            tmp.insert((k.0, k.1 + 1));
            tmp.insert((k.0 + 1, k.1 + 1));
        }
        newtiles.insert(&(0, 0), Tile::new(0, 0).rand());
    }

    pub fn end(&mut self) { self.tiles.clear(); }
}
