use super::{field::Field, logic::Rule, Point};

use hilbert::Point as HPoint;
use indexmap::Equivalent;
use num_bigint::BigUint;
use std::{
    cmp::{min, Ordering},
    collections::HashSet,
    convert::TryFrom,
    hash::{Hash, Hasher},
    ops::{Add, AddAssign},
};

#[derive(Eq, Clone, Debug, Default)]
pub struct Tile {
    pub pos:     Point<u8>,
    pub members: u16,
    pub rule:    Option<Rule>,
    fields:      HashSet<Field>,
}

impl Tile {
    pub fn new(&pos: &Point<u8>) -> Self {
        let members = 0;
        let fields = HashSet::new();
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
        match self.fields.get(&field) {
            Some(&f) => {
                self.fields.insert(f + field);
            }
            None => {
                self.fields.insert(field);
                self.members += 1;
            }
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
            .hilbert_transform(8)
            .to_radix_be(8);

        TryFrom::try_from(BigUint::from_radix_be(&a, 8).unwrap())
            .expect("Tile hilbert index overflow?")
    }

    pub fn test(&mut self) -> Self {
        use rand::Rng;
        for i in 1..4 {
            self.add_field(Field(i + 1, rand::thread_rng().gen_range(1..8)));
        }
        self.clone()
    }

    pub fn color(&self) -> [f32; 4] {
        let mut c: [f32; 4] = [1., 0., 0., 1.];
        if self.members != 0 {
            let v = self
                .fields
                .iter()
                .take(self.fields.len())
                .map(|&a| a.1 as f32)
                .collect::<Vec<f32>>();
            let s = min(v.len(), 3);
            let m = v[0..s].iter().fold(0f32, |mut acc, &a| {
                acc += a;
                acc
            });
            // dbg!(&m);
            c[1..=s].copy_from_slice(&v[0..s]);
            c[1..=s].iter_mut().for_each(|a| *a = (*a % 255.) / 255.);
        }
        c
    }

    pub fn pos(&self) -> usize { Point::<u8>::pos(&self.pos) }
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
impl Equivalent<Point<u8>> for Tile {
    fn equivalent(
        &self,
        key: &Point<u8>,
    ) -> bool {
        &self.pos == key
    }
}
impl Equivalent<Tile> for Point<u8> {
    fn equivalent(
        &self,
        key: &Tile,
    ) -> bool {
        self == &key.pos
    }
}
impl Add<Field> for Tile {
    type Output = Tile;

    fn add(
        self,
        rhs: Field,
    ) -> Self::Output {
        let mut s = self;
        match s.fields.get(&rhs) {
            Some(&f) => {
                s.fields.replace(f + rhs);
            }
            None => {
                s.fields.insert(rhs);
                s.members += 1;
            }
        }
        s
    }
}
impl AddAssign<Field> for Tile {
    fn add_assign(
        &mut self,
        rhs: Field,
    ) {
        match self.fields.get(&rhs) {
            Some(&f) => {
                self.fields.replace(f + rhs);
            }
            None => {
                self.fields.insert(rhs);
                self.members += 1;
            }
        }
    }
}
