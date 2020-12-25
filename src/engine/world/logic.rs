use self::Rule::*;
use super::field::Field;
use crate::functions::{from_json, read_file};

use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use std::collections::HashMap;

#[derive(Eq, PartialEq, Clone, Debug, Serialize, Deserialize)]
pub enum Rule {
    Spread,
    Collect,
}
#[serde_as]
#[derive(Serialize, Deserialize)]
pub struct LogicHandler {
    ruleset: HashMap<Field, Rule>,
}
impl LogicHandler {
    pub fn evaluate(
        &mut self,
        field: &Field,
        other: &Field,
    ) {
        if self.ruleset.contains_key(field) && self.ruleset.contains_key(other) {
            match (self.ruleset.get(field), self.ruleset.get(other)) {
                (Some(&Spread), _) => {}
                (Some(&Collect), _) => {}
                (None, _) => {}
            }
        }
    }

    pub fn save_ruleset(&self) {
        serde_json::to_writer(
            &read_file("assets/config/ruleset.json".to_string()),
            &self.ruleset,
        )
        .expect("Couldn't write json to keymap.");
    }

    pub fn load_ruleset(&mut self) {
        let path = "assets/config/keymap.json".to_string();
        if let Ok(r) = serde_json::from_str(&from_json(path)) {
            self.ruleset = r;
        };
    }
}
