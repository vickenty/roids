use input::Input;

#[derive(Copy, Clone, PartialEq)]
pub enum State {
    Alive,
    Dead,
}

pub trait Entity {
    fn think(&mut self, dt: f32, &Input, born: &mut Vec<Box<Entity>>) -> State;
    fn collide(&mut self, other: &mut Entity);
    fn take_damage(&mut self, f32);
}

pub struct Engine {
    entities: Vec<Box<Entity>>,

    born: Vec<Box<Entity>>,
    dead: Vec<usize>,
}

impl Engine
{
    pub fn new() -> Engine {
        Engine {
            entities: Vec::new(),
            born: Vec::new(),
            dead: Vec::new(),
        }
    }

    pub fn add(&mut self, entity: Box<Entity>) {
        self.entities.push(entity);
    }

    pub fn think(&mut self, dt: f32, input: &Input) {
        for (i, e) in self.entities.iter_mut().enumerate() {
            let state = e.think(dt, input, &mut self.born);
            if state == State::Dead {
                self.dead.push(i);
            }
        }

        while let Some(i) = self.dead.pop() {
            self.entities.swap_remove(i);
        }

        while let Some(e) = self.born.pop() {
            self.entities.push(e);
        }
    }
}
