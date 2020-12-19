pub mod field;
pub mod logic;
pub mod tile;

use self::tile::Tile;

use indexmap::IndexSet;

pub struct World {
    pub tiles: IndexSet<Tile>,
    changed:   bool,
}

impl World {
    pub fn new() -> Self {
        let tiles = IndexSet::<Tile>::new();
        let changed = true;
        Self { tiles, changed }
    }

    // pub fn insert_mut(
    //     &mut self,
    //     tile: Tile,
    // ) {
    //     match self.tiles.entry(tile) {
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
        self.tiles.insert(tile);
    }

    pub fn remove(
        &mut self,
        tile: &Tile,
    ) {
        self.tiles.remove(tile);
        self.changed = true;
    }

    pub fn put(
        &mut self,
        tile: Tile,
    ) {
        let mut t = tile;
        self.tiles.insert(t.test());
        self.changed = true;
    }

    pub fn test(&mut self) -> &mut Self {
        for x in u16::MAX - 100..u16::MAX {
            for y in u16::MAX - 100..u16::MAX {
                self.insert(Tile::new(x, y).test());
            }
        }
        self.changed = true;
        self
    }

    pub fn update(&mut self) {
        // let mut tmp = IndexSet::new();
        // let mut newtiles = IndexSet::new();
        // for v in self.tiles.iter() {
        //     if v.x > 0 && v.y > 0 && v.x < u16::MAX && v.y < u16::MAX {
        //         tmp.insert((v.x - 1, v.y - 1));
        //         tmp.insert((v.x, v.y - 1));
        //         tmp.insert((v.x + 1, v.y - 1));
        //         tmp.insert((v.x - 1, v.y));
        //         tmp.insert((v.x + 1, v.y));
        //         tmp.insert((v.x - 1, v.y + 1));
        //         tmp.insert((v.x, v.y + 1));
        //         tmp.insert((v.x + 1, v.y + 1));
        //     }
        // }
        // newtiles.insert(Tile::new(0, 0).test());
        if self.changed {
            self.hilbert();
            self.changed = false;
        }
    }

    pub fn end(&mut self) { self.tiles.clear(); }

    pub fn hilbert(&mut self) {
        self.tiles.par_sort_by(|left, right| left.cmp(&right));
    }
}
