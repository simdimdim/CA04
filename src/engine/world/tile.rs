use std::{
    cmp::{max, min},
    convert::TryInto,
    ops::{Add, AddAssign, Sub, SubAssign},
};

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Tile {
    pub x:       u16,
    pub y:       u16,
    pub types:   [u32; 16],
    pub values:  [u32; 16],
    pub members: u16,
}
impl Tile {
    pub fn new(
        x: u16,
        y: u16,
    ) -> Self {
        let types = [0; 16];
        let values = [0; 16];
        let members = 0;
        Self {
            x,
            y,
            types,
            values,
            members,
        }
    }

    pub fn coords(&self) -> (u16, u16) { (self.x, self.y) }

    pub fn rand(&mut self) -> Self {
        use rand::Rng;
        for i in 0..4 {
            self.types[i] = i.try_into().unwrap();
            self.values[i] = rand::thread_rng().gen_range(1, 8);
            self.members = 4;
        }
        self.clone()
    }
}
impl Add for Tile {
    type Output = Self;

    fn add(
        self,
        other: Self,
    ) -> Self {
        let mut types = [0u32; 16];
        let mut values = [0u32; 16];
        let mut members = self.members;
        let count = max(members, other.members) as usize;
        for i in 0..count {
            types[i] = if types[i] != other.types[i] {
                if types[i] != 0 {
                    types[i]
                } else {
                    values[i] = other.values[i];
                    other.types[i]
                }
            } else {
                values[i] = values[i].saturating_add(other.values[i]);
                types[i]
            };
        }
        members = count as u16;
        Self {
            x: self.x,
            y: self.y,
            types,
            values,
            members,
        }
    }
}
impl Sub for Tile {
    type Output = Self;

    fn sub(
        self,
        other: Self,
    ) -> Self {
        let mut types = [0u32; 16];
        let mut values = [0u32; 16];
        let mut members = self.members;
        let count = min(members, other.members) as usize;
        let mut n = 0;
        let mut idx;
        for i in 0..count {
            idx = i - n;
            if types[i] != other.types[i] {
                types[idx] = types[i];
                values[idx] = values[i];
            } else {
                let res = values[i].saturating_sub(other.values[i]);
                if res != 0 {
                    types[idx] = types[i];
                    values[idx] = res;
                } else {
                    n += 1;
                    members -= 1
                }
            };
        }
        if n != 0 {
            for m in count - n..count {
                types[m] = 0;
                values[m] = 0;
            }
            // unstable
            // types[count - n..].fill(0);
            // values[count - n..].fill(0);
            types[count - n..].rotate_left(n);
            values[count - n..].rotate_left(n);
        }
        Self {
            x: self.x,
            y: self.y,
            types,
            values,
            members,
        }
    }
}
impl AddAssign for Tile {
    fn add_assign(
        &mut self,
        other: Self,
    ) {
        let Self {
            x,
            y,
            mut types,
            mut values,
            mut members,
        } = self;
        let count = max(members, other.members) as usize;
        for i in 0..count {
            types[i] = if types[i] != other.types[i] {
                if types[i] != 0 {
                    types[i]
                } else {
                    values[i] = other.values[i];
                    other.types[i]
                }
            } else {
                values[i] = values[i].saturating_add(other.values[i]);
                types[i]
            };
        }
        members = count as u16;
        *self = Self {
            x: *x,
            y: *y,
            types,
            values,
            members,
        };
    }
}
impl SubAssign for Tile {
    fn sub_assign(
        &mut self,
        other: Self,
    ) {
        let Self {
            x,
            y,
            mut types,
            mut values,
            mut members,
        } = self;
        let count = min(members, other.members) as usize;
        let mut n = 0usize;
        let mut idx;
        for i in 0..count {
            idx = i - n;
            if types[i] != other.types[i] {
                types[idx] = types[i];
                values[idx] = values[i];
            } else {
                let res = values[i].saturating_sub(other.values[i]);
                if res != 0 {
                    types[idx] = types[i];
                    values[idx] = res;
                } else {
                    n += 1;
                    members -= 1
                }
            };
        }
        if n != 0 {
            for m in count - n..count {
                types[m] = 0;
                values[m] = 0;
            }
            // unstable
            // types[count - n..].fill(0);
            // values[count - n..].fill(0);
            types[count - n..].rotate_left(n);
            values[count - n..].rotate_left(n);
        }
        /* //alt
         * let lower = count - n;
         * for i in lower..count {
         *     types[i] = 0;
         *     values[i] = 0;
         * }
         * types[lower..].rotate_left(n);
         * values[lower..].rotate_left(n);
         */
        *self = Self {
            x: *x,
            y: *y,
            types,
            values,
            members,
        };
    }
}
