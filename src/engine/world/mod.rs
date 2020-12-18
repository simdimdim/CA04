pub mod field;
pub mod logic;
pub mod tile;

use self::tile::Tile;
use indexmap::{IndexMap, IndexSet};

pub struct World {
    pub tiles: IndexMap<(u16, u16), Tile>,
}

impl World {
    pub fn new() -> Self {
        let tiles = IndexMap::<(u16, u16), Tile>::new();
        Self { tiles }
    }

    // pub fn insert_mut(
    //     &mut self,
    //     tile: Tile,
    // ) {
    //     match self.tiles.entry((tile.x, tile.y)) {
    //         hash_map::Entry::Occupied(e) => *e.into_mut() += tile,
    //         hash_map::Entry::Vacant(e) => {
    //             e.insert(tile);
    //         }
    //     }
    // }

    pub fn insert(
        &mut self,
        tile: Tile,
    ) {
        self.tiles.insert((tile.x, tile.y), tile);
    }

    pub fn remove(
        &mut self,
        tile: &(u16, u16),
    ) {
        self.tiles.remove(tile);
    }

    pub fn put(
        &mut self,
        tile: (u16, u16),
    ) {
        self.tiles.insert(tile, Tile::new(tile.0, tile.1).test());
    }

    pub fn test(&mut self) {
        for i in 0..10 {
            self.insert(Tile::new(u16::MAX, i).test());
        }
    }

    pub fn update(&mut self) {
        let mut tmp = IndexSet::new();
        let mut newtiles = IndexMap::new();
        for (k, _v) in self.tiles.iter() {
            if k.0 > 0 && k.1 > 0 && k.0 < u16::MAX && k.1 < u16::MAX {
                tmp.insert((k.0 - 1, k.1 - 1));
                tmp.insert((k.0, k.1 - 1));
                tmp.insert((k.0 + 1, k.1 - 1));
                tmp.insert((k.0 - 1, k.1));
                tmp.insert((k.0 + 1, k.1));
                tmp.insert((k.0 - 1, k.1 + 1));
                tmp.insert((k.0, k.1 + 1));
                tmp.insert((k.0 + 1, k.1 + 1));
            }
        }
        newtiles.insert(&(0, 0), Tile::new(0, 0).test());
    }

    pub fn end(&mut self) { self.tiles.clear(); }
}
