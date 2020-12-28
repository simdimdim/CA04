use super::{field::Field, logic::Rule};

use hilbert::Point as HPoint;
use indexmap::{Equivalent, IndexSet};
use num_bigint::BigUint;
use serde::{Deserialize, Serialize};
use std::{
    cmp::Ordering,
    convert::TryFrom,
    hash::{Hash, Hasher},
};

#[derive(Eq, Hash, PartialEq, Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Point(pub u16, pub u16);
impl Equivalent<Tile> for Point {
    fn equivalent(
        &self,
        key: &Tile,
    ) -> bool {
        self == &key.pos
    }
}

#[derive(Eq, Clone, Debug)]
pub struct Tile {
    pub pos:     Point,
    pub members: u16,
    pub rule:    Option<Rule>,
    fields:      IndexSet<Field>,
}

impl Tile {
    pub fn new(&pos: &Point) -> Self {
        let members = 0;
        let fields = IndexSet::new();
        let rule = None;
        Self {
            pos,
            members,
            rule,
            fields,
        }
    }

    pub fn add_field(
        &mut self,
        field: Field,
    ) {
        if self.fields.insert(field) {
            self.members += 1;
        }
    }

    pub fn remove_field(
        &mut self,
        field: Field,
    ) {
        if self.fields.remove(&field) {
            self.members -= 1;
        }
    }

    pub fn hilbert_index(
        &self,
        &i: &usize,
    ) -> u64 {
        let a = HPoint::new(i, &[self.pos.0 as u32, self.pos.1 as u32])
            .hilbert_transform(32)
            .to_radix_be(32);

        TryFrom::try_from(BigUint::from_radix_be(&a, 32).unwrap())
            .expect("Tile hilbert index overflow?")
    }

    pub fn on_screen(
        &self,
        width: f64,
        height: f64,
        offset_x: f64,
        offset_y: f64,
    ) -> bool {
        (self.pos.0 as f64) >= -offset_x &&
            (self.pos.1 as f64) >= -offset_y &&
            (self.pos.0 as f64) < -offset_x + width &&
            (self.pos.1 as f64) < -offset_y + height
    }

    pub fn test(&mut self) -> Self {
        use rand::Rng;
        for i in 0..4 {
            self.add_field(Field(i + 1, rand::thread_rng().gen_range(1..8)));
        }
        self.clone()
    }
}

impl Ord for Tile {
    fn cmp(
        &self,
        other: &Self,
    ) -> Ordering {
        self.hilbert_index(&0).cmp(&&other.hilbert_index(&1))
    }
}
impl PartialOrd for Tile {
    fn partial_cmp(
        &self,
        other: &Self,
    ) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialEq for Tile {
    fn eq(
        &self,
        other: &Self,
    ) -> bool {
        self.pos == other.pos
    }
}
impl Hash for Tile {
    fn hash<H: Hasher>(
        &self,
        state: &mut H,
    ) {
        self.pos.hash(state);
    }
}
impl Equivalent<Point> for Tile {
    fn equivalent(
        &self,
        key: &Point,
    ) -> bool {
        &self.pos == key
    }
}
