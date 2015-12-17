use input::Input;
use render::{ Batch, Resources };

#[derive(Copy, Clone, PartialEq)]
pub enum State {
    Alive,
    Dead,
}

pub trait Entity<R> where R: Resources {
    fn think(&mut self, dt: f32, &Input, born: &mut Vec<Box<Entity<R>>>) -> State;
    fn collide(&mut self, other: &mut Entity<R>);
    fn take_damage(&mut self, f32);
    fn get_batch(&mut self) -> &Batch<R>;
}

pub struct Engine<R> {
    entities: Vec<Box<Entity<R>>>,

    born: Vec<Box<Entity<R>>>,
    dead: Vec<usize>,
}

impl<R> Engine<R>
    where R: Resources
{
    pub fn new() -> Engine<R> {
        Engine {
            entities: Vec::new(),
            born: Vec::new(),
            dead: Vec::new(),
        }
    }

    pub fn add(&mut self, entity: Box<Entity<R>>) {
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
