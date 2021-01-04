use super::{tile::Tile, Point};

use array_macro::array;
use hilbert::Point as HPoint;
use indexmap::Equivalent;
use num_bigint::BigUint;
use std::{
    cmp::Ordering,
    convert::TryFrom,
    hash::{Hash, Hasher},
};

#[derive(Eq, Clone, Debug)]
pub struct Chunk {
    pub pos:     Point<u16>,
    pub tiles:   [Tile; 1024],
    pub border:  [Option<Tile>; 128],
    pub changed: bool,
}

impl Default for Chunk {
    fn default() -> Self {
        let tiles = array![x=>Tile::new(&Point::from(x)); 1024];
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
        camera: (f64, f64, f64, f64),
    ) -> bool {
        self.pos.0 >= (camera.0 - camera.2) as u16 &&
            self.pos.1 >= (camera.1 - camera.3) as u16 &&
            self.pos.0 <= (camera.0 + camera.2) as u16 &&
            self.pos.1 <= (camera.1 + camera.3) as u16
        // true
    }

    pub fn hilbert_index(
        &self,
        &i: &usize,
    ) -> u64 {
        let a = HPoint::new(i, &[self.pos.0 as u32, self.pos.1 as u32])
            .hilbert_transform(16)
            .to_radix_be(16);
        TryFrom::try_from(BigUint::from_radix_be(&a, 16).unwrap())
            .expect("Tile hilbert index overflow?")
    }
}

impl Ord for Chunk {
    fn cmp(
        &self,
        other: &Self,
    ) -> Ordering {
        self.hilbert_index(&0).cmp(&&other.hilbert_index(&1))
    }
}
impl PartialOrd for Chunk {
    fn partial_cmp(
        &self,
        other: &Self,
    ) -> Option<Ordering> {
        Some(self.cmp(other))
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
