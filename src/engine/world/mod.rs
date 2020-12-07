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
}
