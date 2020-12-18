use serde::{Deserialize, Serialize};
use std::{
    cmp::Ordering,
    hash::{Hash, Hasher},
    ops::{Add, AddAssign, Sub, SubAssign},
};

#[derive(Eq, Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Field(pub u32, pub u32);
impl Ord for Field {
    fn cmp(
        &self,
        other: &Self,
    ) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl PartialOrd for Field {
    fn partial_cmp(
        &self,
        other: &Self,
    ) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialEq for Field {
    fn eq(
        &self,
        other: &Self,
    ) -> bool {
        self.0 == other.0
    }
}
impl Hash for Field {
    fn hash<H: Hasher>(
        &self,
        state: &mut H,
    ) {
        self.0.hash(state);
    }
}
impl Add for Field {
    type Output = Self;

    fn add(
        self,
        rhs: Self,
    ) -> Self::Output {
        if self == rhs {
            Self(self.0, self.1.saturating_add(rhs.1))
        } else {
            self
        }
    }
}
impl Sub for Field {
    type Output = Self;

    fn sub(
        self,
        rhs: Self,
    ) -> Self::Output {
        if self == rhs {
            Self(self.0, self.1.saturating_sub(rhs.1))
        } else {
            self
        }
    }
}
impl AddAssign for Field {
    fn add_assign(
        &mut self,
        other: Self,
    ) {
        if *self == other {
            *self = Self(self.0, self.1.saturating_add(other.1))
        }
    }
}
impl SubAssign for Field {
    fn sub_assign(
        &mut self,
        other: Self,
    ) {
        if *self == other {
            *self = Self(self.0, self.1.saturating_sub(other.1))
        }
    }
}
