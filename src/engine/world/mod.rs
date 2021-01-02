pub mod chunk;
pub mod field;
pub mod logic;
pub mod tile;

use self::{chunk::Chunk, field::Field, tile::Tile};

use hilbert::Point as HPoint;
use indexmap::IndexMap;
use logic::Rule::*;
use num_bigint::BigUint;
use serde::{Deserialize, Serialize};
use std::{
    cmp::Ordering,
    collections::HashSet,
    convert::TryFrom,
    ops::{Add, Mul},
};

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
        pos: &Point<Point<u16>, usize>,
    ) {
        if let Some(chunk) = self.chunks.get_mut(&pos.0) {
            chunk.tiles[pos.1] = Tile::new(&pos.1.into());
            self.changed = true;
        }
    }

    pub fn put(
        &mut self,
        coords: &Point<Point<u16>, usize>,
    ) {
        if let Some(chunk) = self.chunks.get_mut(&coords.0) {
            chunk.tiles[coords.1].pos = coords.1.into();
            chunk.tiles[coords.1].add_field(Field(1, 10));
        } else {
            let mut chunk = Chunk::default();
            chunk.pos = coords.0;
            chunk.tiles[coords.1].pos = coords.1.into();
            chunk.tiles[coords.1].add_field(Field(1, 10));
            self.chunks.insert(coords.0, chunk);
            self.changed = true;
        }
    }

    pub fn update(&mut self) {
        let mut nb = HashSet::new();
        self.chunks.iter().for_each(|(&p, chunk)| {
            // .filter(|&(_, t)| t.changed)
            nb = chunk.tiles.iter().enumerate().fold(
                HashSet::new(),
                |mut neighbours, (i, _)| {
                    match (i / 32, i % 32) {
                        // top left
                        (0, 0) => {
                            // neighbours.insert(Point(p.neighb(-1, -1),
                            // 1023));
                            // neighbours.insert(Point(p.neighb(0, -1), 992));
                            // neighbours.insert(Point(p.neighb(0, -1), 993));

                            // neighbours.insert(Point(p.neighb(-1, 0), 63));
                            neighbours.insert(Point(p, i + 1));
                            // neighbours.insert(Point(p.neighb(-1, 0), 31));
                            // neighbours.insert(Point(p, i + 32));
                            // neighbours.insert(Point(p, i + 33));
                            neighbours
                        }
                        // top right
                        (00, 31) => {
                            // neighbours.insert(Point(p.neighb(0, -1),
                            // 1022));
                            // neighbours.insert(Point(p.neighb(0, -1),
                            // 1023));
                            // neighbours.insert(Point(p.neighb(1, -1), 992));
                            // neighbours.insert(Point(p, i - 1));
                            neighbours.insert(Point(p.neighb(1, 0), 0));
                            // neighbours.insert(Point(p, i + 31));
                            // neighbours.insert(Point(p, i + 32));
                            // neighbours.insert(Point(p.neighb(1, 0), 32));
                            neighbours
                        }
                        // bottom left
                        (31, 00) => {
                            // neighbours.insert(Point(p.neighb(-1, 0), 991));
                            // neighbours.insert(Point(p, i - 32));
                            // neighbours.insert(Point(p, i - 31));
                            // neighbours.insert(Point(p.neighb(-1, 0),
                            // 1023));
                            neighbours.insert(Point(p, i + 1));
                            // neighbours.insert(Point(p.neighb(-1, 1), 1));
                            // neighbours.insert(Point(p.neighb(0, 1), 0));
                            // neighbours.insert(Point(p.neighb(0, 1), 31));
                            neighbours
                        }
                        // bottom right
                        (31, 31) => {
                            // neighbours.insert(Point(p, i - 33));
                            // neighbours.insert(Point(p, i - 32));
                            // neighbours.insert(Point(p.neighb(1, 0), 960));
                            // neighbours.insert(Point(p, i - 1));
                            neighbours.insert(Point(p.neighb(1, 0), 992));
                            // neighbours.insert(Point(p.neighb(0, 1), 1022));
                            // neighbours.insert(Point(p.neighb(0, 1), 1023));
                            // neighbours.insert(Point(p.neighb(1, 1), 0));
                            neighbours
                        }
                        // top
                        (00, _) => {
                            // neighbours
                            //     .insert(Point(p.neighb(0, -1), 991 + i));
                            // neighbours
                            //     .insert(Point(p.neighb(0, -1), 992 + i));
                            // neighbours
                            //     .insert(Point(p.neighb(0, -1), 993 + i));
                            // neighbours.insert(Point(p, i - 1));
                            neighbours.insert(Point(p, i + 1));
                            // neighbours.insert(Point(p, i + 31));
                            // neighbours.insert(Point(p, i + 32));
                            // neighbours.insert(Point(p, i + 33));
                            neighbours
                        }
                        // left
                        (_, 00) => {
                            // neighbours.insert(Point(p.neighb(-1, 0), i -
                            // 1));
                            // neighbours.insert(Point(p, i - 32));
                            // neighbours.insert(Point(p, i - 31));
                            // neighbours.insert(Point(p.neighb(-1, 0), i +
                            // 31));
                            neighbours.insert(Point(p, i + 1));
                            // neighbours.insert(Point(p.neighb(-1, 0), i +
                            // 63));
                            // neighbours.insert(Point(p, i + 32));
                            // neighbours.insert(Point(p, i + 33));
                            neighbours
                        }
                        // right
                        (_, 31) => {
                            // neighbours.insert(Point(p, i - 33));
                            // neighbours.insert(Point(p, i - 32));
                            // neighbours.insert(Point(p.neighb(1, 0), i -
                            // 63));
                            // neighbours.insert(Point(p, i - 1));
                            neighbours.insert(Point(p.neighb(1, 0), i - 31));
                            // neighbours.insert(Point(p, i + 31));
                            // neighbours.insert(Point(p, i + 32));
                            // neighbours.insert(Point(p.neighb(1, 0), i +
                            // 1));
                            neighbours
                        }
                        // bottom
                        (31, _) => {
                            // neighbours.insert(Point(p, i - 33));
                            // neighbours.insert(Point(p, i - 32));
                            // neighbours.insert(Point(p, i - 31));
                            // neighbours.insert(Point(p, i - 1));
                            neighbours.insert(Point(p, i + 1));
                            // neighbours.insert(Point(p.neighb(0, 1), i -
                            // 993));
                            // neighbours.insert(Point(p.neighb(0, 1), i -
                            // 992));
                            // neighbours.insert(Point(p.neighb(0, 1), i -
                            // 991));
                            neighbours
                        }
                        // inside
                        (_, _) => {
                            // neighbours.insert(Point(p, i - 33));
                            // neighbours.insert(Point(p, i - 32));
                            // neighbours.insert(Point(p, i - 31));
                            // neighbours.insert(Point(p, i - 1));
                            neighbours.insert(Point(p, i + 1));
                            // neighbours.insert(Point(p, i + 31));
                            // neighbours.insert(Point(p, i + 32));
                            // neighbours.insert(Point(p, i + 33));
                            neighbours
                        }
                    }
                },
            )
        });
        // dbg!("thus");
        nb.iter().for_each(|p| match self.chunks.get_mut(&p.0) {
            Some(ch) => {
                ch.tiles[p.1].add_field(Field(1, 10));
            }
            None => self.put(p),
        });

        if self.changed {
            self.hilbert();
            self.changed = false;
        }
    }

    pub fn end(&mut self) { self.chunks.clear(); }

    pub fn hilbert(&mut self) {
        self.chunks.par_sort_by(|k1, _, k2, _| k1.cmp(&k2));
    }
}

impl Point<u8> {
    pub fn pos(&self) -> usize { (self.0 as u16 * 32 + self.1 as u16) as usize }

    pub fn hilbert_index(
        &self,
        &i: &usize,
    ) -> u64 {
        let a = HPoint::new(i, &[self.0 as u32, self.1 as u32])
            .hilbert_transform(8)
            .to_radix_be(8);
        TryFrom::try_from(BigUint::from_radix_be(&a, 8).unwrap())
            .expect("Tile hilbert index overflow?")
    }

    pub fn neighb(
        &self,
        x: i8,
        y: i8,
    ) -> Point<u8> {
        Point(self.0.wrapping_add(x as u8), self.1.wrapping_add(y as u8))
    }
}
impl Point<u16> {
    pub fn pos(&self) -> usize { (self.0 * 32 + self.1) as usize }

    pub fn hilbert_index(
        &self,
        &i: &usize,
    ) -> u64 {
        let a = HPoint::new(i, &[self.0 as u32, self.1 as u32])
            .hilbert_transform(16)
            .to_radix_be(16);
        TryFrom::try_from(BigUint::from_radix_be(&a, 16).unwrap())
            .expect("Tile hilbert index overflow?")
    }

    pub fn neighb(
        &self,
        x: i16,
        y: i16,
    ) -> Point<u16> {
        Point(self.0.wrapping_add(x as u16), self.1.wrapping_add(y as u16))
    }
}
impl Point<f64> {
    pub fn pos(&self) -> usize { (self.0 * 32. + self.1) as usize }
}
impl Ord for Point<u8> {
    fn cmp(
        &self,
        other: &Self,
    ) -> Ordering {
        self.hilbert_index(&0).cmp(&&other.hilbert_index(&1))
    }
}
impl Ord for Point<u16> {
    fn cmp(
        &self,
        other: &Self,
    ) -> Ordering {
        self.hilbert_index(&0).cmp(&&other.hilbert_index(&1))
    }
}
impl PartialOrd for Point<u8> {
    fn partial_cmp(
        &self,
        other: &Self,
    ) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialOrd for Point<u16> {
    fn partial_cmp(
        &self,
        other: &Self,
    ) -> Option<Ordering> {
        Some(self.cmp(other))
    }
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
impl From<usize> for Point<u8> {
    fn from(i: usize) -> Self { Point::<u8>((i / 32) as u8, (i % 32) as u8) }
}
impl From<Point<u8>> for Point<f64> {
    fn from(p: Point<u8>) -> Self { Point(p.0 as f64, p.1 as f64) }
}
impl From<Point<u16>> for Point<f64> {
    fn from(p: Point<u16>) -> Self { Point(p.0 as f64, p.1 as f64) }
}
