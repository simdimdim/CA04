use piston_window::{
    Button,
    Event,
    Key,
    MouseButton,
    MouseCursorEvent,
    PressEvent,
    ReleaseEvent,
};
use std::{
    collections::{btree_map::Values, BTreeMap, BTreeSet, HashMap},
    time::{Duration, Instant},
};

use crate::engine::input::{Action::*, MouseA::*};

#[derive(Debug, Clone)]
pub struct InputHandler {
    mouse:      BTreeMap<Button, MouseA>,
    down:       BTreeSet<Button>,
    last:       BTreeSet<Button>,
    repeat:     bool,
    drag:       bool,
    delay:      Duration,
    time:       Instant,
    keymap:     HashMap<BTreeSet<Button>, Action>,
    pub cursor: [f64; 2],
}
#[derive(Clone, Debug)]
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
#[derive(Clone, Debug)]
pub enum MouseA {
    LMB(f64, f64),
    RMB(f64, f64),
    MMB(f64, f64),
    Drag(f64, f64, f64, f64),
}
impl InputHandler {
    pub fn new() -> Self {
        let mouse = BTreeMap::new();
        let down = BTreeSet::new();
        let last = BTreeSet::new();
        let repeat = false;
        let drag = false;
        let delay = Duration::new(0, 250_000_000);
        let time = Instant::now();
        let keymap = HashMap::new();
        let cursor = [0.; 2];
        Self {
            mouse,
            down,
            last,
            repeat,
            drag,
            delay,
            time,
            keymap,
            cursor,
        }
    }

    pub fn event(
        &mut self,
        e: &Event,
    ) -> &Action {
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
                    self.drag = false
                }
            }
        };
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
                if mouse_button == MouseButton::Right {
                    // world.remove(world.grid.get_pos(cursor[0],
                    // cursor[1]));
                }
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

    pub fn mouse(&self) -> Values<'_, Button, MouseA> { self.mouse.values() }

    pub fn repeat(&self) -> bool { self.repeat }

    pub fn populate(&mut self) {
        // TODO: load from config

        let mut k = BTreeSet::new();
        k.insert(Button::Keyboard(Key::Q));
        self.keymap.insert(k.clone(), Exit);
        k.clear();
        k.insert(Button::Keyboard(Key::E));
        self.keymap.insert(k.clone(), Stats);
        k.clear();
        k.insert(Button::Keyboard(Key::R));
        self.keymap.insert(k.clone(), ResetZoom);
        k.clear();
        k.insert(Button::Keyboard(Key::W));
        self.keymap.insert(k.clone(), N);
        k.clear();
        k.insert(Button::Keyboard(Key::S));
        self.keymap.insert(k.clone(), S);
        k.clear();
        k.insert(Button::Keyboard(Key::A));
        self.keymap.insert(k.clone(), W);
        k.clear();
        k.insert(Button::Keyboard(Key::D));
        self.keymap.insert(k.clone(), E);
        k.clear();
        k.insert(Button::Keyboard(Key::W));
        k.insert(Button::Keyboard(Key::D));
        self.keymap.insert(k.clone(), NE);
        k.clear();
        k.insert(Button::Keyboard(Key::W));
        k.insert(Button::Keyboard(Key::A));
        self.keymap.insert(k.clone(), NW);
        k.clear();
        k.insert(Button::Keyboard(Key::S));
        k.insert(Button::Keyboard(Key::A));
        self.keymap.insert(k.clone(), SW);
        k.clear();
        k.insert(Button::Keyboard(Key::S));
        k.insert(Button::Keyboard(Key::D));
        self.keymap.insert(k.clone(), SE);
    }
}
