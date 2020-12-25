use crate::{
    engine::input::{Action::*, MouseB::*, MouseM::*},
    functions::{from_json, read_file},
};

use piston_window::{
    Button,
    Event,
    MouseButton,
    MouseCursorEvent,
    MouseScrollEvent,
    PressEvent,
    ReleaseEvent,
};
use serde::{Deserialize, Serialize};
use serde_with::{json::JsonString, serde_as};
use std::{
    collections::{btree_map, BTreeMap, BTreeSet, HashMap},
    time::{Duration, Instant},
};

#[derive(Debug, Clone)]
pub struct InputHandler {
    mouse:      BTreeMap<Button, MouseB>,
    motion:     [Option<MouseM>; 2],
    down:       BTreeSet<Button>,
    last:       BTreeSet<Button>,
    repeat:     bool,
    drag:       bool,
    delay:      Duration,
    time:       Instant,
    keymap:     HashMap<BTreeSet<Button>, Action>,
    pub cursor: [f64; 2],
    scroll:     bool,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Action {
    Pass,
    Exit,
    Stats,
    ResetZoom,
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}
#[derive(Copy, Clone, Debug)]
pub enum MouseB {
    LMB(f64, f64),
    RMB(f64, f64),
    MMB(f64, f64),
}
#[derive(Copy, Clone, Debug)]
pub enum MouseM {
    Scroll(f64),
    Drag(f64, f64, f64, f64),
}

impl InputHandler {
    pub fn new() -> Self {
        let mouse = BTreeMap::new();
        let motion = [None; 2];
        let down = BTreeSet::new();
        let last = BTreeSet::new();
        let repeat = false;
        let drag = false;
        let delay = Duration::new(0, 250_000_000);
        let time = Instant::now();
        let keymap = HashMap::new();
        let cursor = [0.; 2];
        let scroll = false;
        Self {
            mouse,
            motion,
            down,
            last,
            repeat,
            drag,
            delay,
            time,
            keymap,
            cursor,
            scroll,
        }
    }

    pub fn event(
        &mut self,
        e: &Event,
    ) -> &Action {
        if self.scroll {
            self.motion[0] = None;
            self.scroll = false;
        }
        match e.mouse_cursor(|xy| xy) {
            Some(pos) => {
                self.cursor = pos;
                if !self.mouse.is_empty() {
                    self.drag = true;
                }
            }
            None => {
                if !self.mouse.is_empty() {
                    self.mouse.clear();
                    self.drag = false;
                }
            }
        };
        e.mouse_scroll(|d| {
            self.motion[0] = Some(Scroll(d[1]));
            self.scroll = true;
        });
        if let Some(button) = e.press_args() {
            if let Button::Keyboard(_) = button {
                self.last.clear();
                self.last.insert(button);
                self.time = Instant::now();
                self.down.insert(button);
            }
            if let Button::Mouse(mouse_button) = button {
                if mouse_button == MouseButton::Left {
                    self.mouse
                        .insert(button, LMB(self.cursor[0], self.cursor[1]));
                }
                if mouse_button == MouseButton::Right {
                    self.mouse
                        .insert(button, RMB(self.cursor[0], self.cursor[1]));
                }
                if mouse_button == MouseButton::Middle {
                    self.mouse
                        .insert(button, MMB(self.cursor[0], self.cursor[1]));
                    // world.remove(world.grid.get_pos(cursor[0],
                    // cursor[1]));
                }
            }
        }
        if let Some(button) = e.release_args() {
            if let Button::Keyboard(_) = button {
                if self.down.contains(&button) {
                    self.down.remove(&button);
                    self.last.clear();
                    self.repeat = false;
                }
            }
            if let Button::Mouse(mouse_button) = button {
                self.mouse.remove(&button);
                if mouse_button == MouseButton::Left {}
                if mouse_button == MouseButton::Right {}
                if mouse_button == MouseButton::Middle {}
            }
        }

        if !self.down.is_empty() {}

        self.repeat = self.time.elapsed() >= self.delay;

        if self.keymap.contains_key(&self.down) {
            self.keymap.get(&self.down).unwrap()
        } else {
            &Pass
        }
    }

    pub fn mouse(&self) -> btree_map::Values<'_, Button, MouseB> {
        self.mouse.values()
    }

    pub fn motion(&self) -> &[Option<MouseM>] { &self.motion }

    pub fn repeat(&self) -> bool { self.repeat }

    pub fn save_keymap(&self) {
        serde_json::to_writer(
            &read_file("assets/config/keymap.json".to_string()),
            &KeyMap(self.keymap.clone()),
        )
        .expect("Couldn't write json to keymap.");
    }

    pub fn load_keymap(&mut self) {
        let path = "assets/config/keymap.json".to_string();
        if let Ok(KeyMap(k)) = serde_json::from_str(&from_json(path)) {
            self.keymap = k;
        };
    }
}

impl PartialEq for MouseM {
    fn eq(
        &self,
        other: &Self,
    ) -> bool {
        fn f(i: &MouseM) -> u8 {
            match i {
                MouseM::Scroll(_) => 0,
                MouseM::Drag(_, _, _, _) => 1,
            }
        }
        f(self) == f(&other)
    }
}
#[serde_as]
#[derive(Serialize, Deserialize)]
struct KeyMap(
    #[serde_as(as = "HashMap<JsonString, _>")] HashMap<BTreeSet<Button>, Action>,
);
