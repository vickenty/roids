use std::collections::HashMap;

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
}

impl Input {
    pub fn pressed(&self, key: Key) -> bool {
        match self.state.get(&key) {
            Some(state_ref) => *state_ref,
            None => false,
        }
    }
}
