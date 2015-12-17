use std::collections::HashMap;
use glutin::{ Event, ElementState, VirtualKeyCode };

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub enum Key {
    Left,
    Right,
    Forward,
    Reverse,
    Fire,
}

pub struct Input {
    state: HashMap<Key, bool>,
    keymap: HashMap<VirtualKeyCode, Key>,
}

impl Input {
    pub fn new() -> Input {
        let mut input = Input {
            state: HashMap::new(),
            keymap: HashMap::new(),
        };

        input.keymap.insert(VirtualKeyCode::Left, Key::Left);
        input.keymap.insert(VirtualKeyCode::Right, Key::Right);
        input.keymap.insert(VirtualKeyCode::Up, Key::Forward);
        input.keymap.insert(VirtualKeyCode::Down, Key::Reverse);
        input.keymap.insert(VirtualKeyCode::Space, Key::Fire);

        input
    }

    pub fn pressed(&self, key: Key) -> bool {
        *self.state.get(&key).unwrap_or(&false)
    }

    fn handle_key(&mut self, el_state: &ElementState, keycode: &VirtualKeyCode) {
        let new_state = *el_state == ElementState::Pressed;

        if let Some(key) = self.keymap.get(keycode) {
            let the_state = self.state.entry(*key).or_insert(false);
            *the_state = new_state;
        }
    }

    pub fn handle_event(&mut self, ev: &Event) {
        match *ev {
            Event::KeyboardInput(ref state, _, Some(ref key)) => self.handle_key(state, key),
            _ => (),
        }
    }
}
