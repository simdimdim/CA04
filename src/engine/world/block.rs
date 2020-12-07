use itertools::izip;
use std::{
    cmp::{max, min},
    ops::{Add, AddAssign, Sub, SubAssign},
    rc::Rc,
};

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Block {
    pub coords:  Rc<(u16, u16)>,
    pub types:   [u32; 64],
    pub values:  [u32; 64],
    pub members: u16,
}
impl Block {
    pub fn new(coords: (u16, u16)) -> Self {
        let coords = Rc::new(coords);
        let types = [0; 64];
        let values = [0; 64];
        Self {
            coords,
            types,
            values,
            members: 0,
        }
    }
}
impl Add for Block {
    type Output = Self;

    fn add(
        self,
        other: Self,
    ) -> Self {
        let mut types = [0u32; 64];
        let mut values = [0u32; 64];
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
            coords: self.coords.clone(),
            types,
            values,
            members,
        }
    }
}
impl Sub for Block {
    type Output = Self;

    fn sub(
        self,
        other: Self,
    ) -> Self {
        let mut types = [0u32; 64];
        let mut values = [0u32; 64];
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
            types[count - n..count].fill(0);
            values[count - n..count].fill(0);
            types[count - n..].rotate_left(n);
            values[count - n..].rotate_left(n);
        }
        Self {
            coords: self.coords,
            types,
            values,
            members,
        }
    }
}
impl AddAssign for Block {
    fn add_assign(
        &mut self,
        other: Self,
    ) {
        let Self {
            coords,
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
            coords: coords.clone(),
            types,
            values,
            members,
        };
    }
}
impl SubAssign for Block {
    fn sub_assign(
        &mut self,
        other: Self,
    ) {
        let Self {
            coords,
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
            types[count - n..count].fill(0);
            values[count - n..count].fill(0);
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
            coords: coords.clone(),
            types,
            values,
            members,
        };
    }
}
