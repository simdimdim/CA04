pub mod chunk;
pub mod field;
pub mod logic;
pub mod tile;

use std::ops::{Add, Mul};

use self::{chunk::Chunk, tile::Tile};
use logic::Rule::*;

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

#[derive(
    Eq, Hash, PartialEq, Copy, Clone, Debug, Default, Serialize, Deserialize,
)]
pub struct Point<T, D = T>(pub T, pub D);

pub struct World {
    pub chunks: IndexMap<Point<u16>, Chunk>,
    changed:    bool,
}

impl World {
    pub fn new() -> Self {
        let chunks = IndexMap::new();
        let changed = true;
        Self { chunks, changed }
    }

    pub fn interract(
        &mut self,
        pos: Point<Point<u16>, Tile>,
    ) {
        match self.chunks.get_mut(&pos.0) {
            Some(chunk) => {
                let Tile {
                    pos: _,
                    members: _,
                    rule,
                    ..
                } = chunk.tiles[pos.1.pos()];
                // *t.into_mut() *= tile;
                match rule.unwrap_or(Collect) {
                    Collect => {}
                    Spread => {}
                    Multiply => {}
                }
            }
            None => {
                // self.chunks.insert(tile);
            }
        }
    }

    pub fn remove(
        &mut self,
        pos: &Point<Point<u16>, Point<u8>>,
    ) {
        if let Some(chunk) = self.chunks.get_mut(&pos.0) {
            chunk.tiles[pos.1.pos()] = Tile::new(&pos.1);
            self.changed = true;
        }
    }

    pub fn put(
        &mut self,
        coords: &Point<Point<u16>, Point<u8>>,
    ) {
        match self.chunks.get_mut(&coords.0) {
            Some(mut chunk) => {
                chunk.tiles[coords.1.pos()].pos = coords.1;
                chunk.tiles[coords.1.pos()].test();
            }
            None => {
                let mut chunk = Chunk::default();
                chunk.pos = coords.0;
                chunk.tiles[coords.1.pos()].pos = coords.1;
                chunk.tiles[coords.1.pos()].test();
                self.chunks.insert(coords.0, chunk);
                self.changed = true;
            }
        }
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
            // self.hilbert();
            self.changed = false;
        }
    }

    pub fn end(&mut self) { self.chunks.clear(); }

    // pub fn hilbert(&mut self) {
    //     self.tiles.par_sort_by(|left, right| left.cmp(&right));
    // }
}
impl Point<u8> {
    pub fn pos(&self) -> usize { (self.0 as u16 * 32 + self.1 as u16) as usize }
}
impl Mul<f64> for Point<u16> {
    type Output = Point<f64>;

    fn mul(
        self,
        rhs: f64,
    ) -> Self::Output {
        Point(self.0 as f64 * rhs, self.1 as f64 * rhs)
    }
}
impl Mul<f64> for Point<f64> {
    type Output = Self;

    fn mul(
        self,
        rhs: f64,
    ) -> Self::Output {
        Point(self.0 as f64 * rhs, self.1 as f64 * rhs)
    }
}
impl Add<Point<u8>> for Point<f64> {
    type Output = Self;

    fn add(
        self,
        rhs: Point<u8>,
    ) -> Self::Output {
        Point(self.0 + rhs.0 as f64, self.1 + rhs.1 as f64)
    }
}
