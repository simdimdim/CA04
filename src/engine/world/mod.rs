pub mod block;
use std::{
    collections::{hash_map, HashMap},
    rc::Rc,
};

use self::block::Block;

pub struct World {
    pub tiles: HashMap<Rc<(u16, u16)>, Block>,
}

impl World {
    pub fn new() -> Self {
        let tiles = HashMap::<Rc<(u16, u16)>, Block>::new();
        Self { tiles }
    }

    pub fn insert_mut(
        &mut self,
        block: Block,
    ) {
        match self.tiles.entry(block.coords.clone()) {
            hash_map::Entry::Occupied(e) => *e.into_mut() += block,
            hash_map::Entry::Vacant(e) => {
                e.insert(block);
            }
        }
    }

    pub fn insert(
        &mut self,
        block: Block,
    ) {
        self.tiles.insert(block.coords.clone(), block);
    }

    pub fn test(&mut self) {
        let mut a = Block {
            coords:  Rc::new((0, 0)),
            types:   [0; 64],
            values:  [0; 64],
            members: 7,
        };
        a.types[0..7].copy_from_slice(&[1, 2, 3, 4, 5, 6, 7]);
        a.values[0..7].copy_from_slice(&[10, 10, 10, 10, 10, 11, 17]);
        let mut b = a.clone();
        b.types[0..7].copy_from_slice(&[1, 2, 3, 4, 7, 0, 0]);
        b.values[0..7].copy_from_slice(&[2, 20, 10, 7, 2, 0, 0]);
        b.members = 5;
        self.insert(a);
        self.insert(b);
    }

    pub fn update(&mut self) {}

    pub fn end(&mut self) { self.tiles = HashMap::new(); }
}
