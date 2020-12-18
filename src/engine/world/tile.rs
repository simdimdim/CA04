use super::field::Field;

use indexmap::IndexSet;
use std::cmp::Ordering;

#[derive(Eq, Clone, Debug)]
pub struct Tile {
    pub x:       u16,
    pub y:       u16,
    pub members: u16,
    fields:      IndexSet<Field>,
}

impl Tile {
    pub fn new(
        x: u16,
        y: u16,
    ) -> Self {
        let members = 0;
        let fields = IndexSet::new();
        Self {
            x,
            y,
            members,
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

    pub fn xy(&self) -> (&u16, &u16) { (&self.x, &self.y) }

    pub fn test(&mut self) -> Self {
        use rand::Rng;
        for i in 0..4 {
            self.add_field(Field(i + 1, rand::thread_rng().gen_range(1, 8)));
        }
        self.clone()
    }
}
impl Ord for Tile {
    fn cmp(
        &self,
        other: &Self,
    ) -> Ordering {
        (self.x + self.y * u16::MAX).cmp(&(&other.x + &other.y * u16::MAX))
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
        self.x == other.x
    }
}
