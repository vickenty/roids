use input::Input;
use render;
use physics;
use hud::Hud;

#[derive(Copy, Clone, PartialEq)]
pub enum State {
    Alive,
    Dead,
}

pub trait Entity {
    fn draw(&mut self, renderer: &mut render::Renderer);
    fn think(&mut self, dt: f32, &Input, hud: &mut Hud, born: &mut Vec<Box<Entity>>) -> State;
    fn collide(&mut self, other: &mut Entity, energy: f32);
    fn body(&mut self) -> Option<&mut physics::Body>;
}

pub struct Engine {
    entities: Vec<Box<Entity>>,

    born: Vec<Box<Entity>>,
    dead: Vec<usize>,
}

fn collide(a: &mut Entity, b: &mut Entity) -> Option<f32> {
    if let (Some(i), Some(j)) = (a.body(), b.body()) {
        physics::collide(i, j)
    } else {
        None
    }
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

    pub fn draw(&mut self, renderer: &mut render::Renderer) {
        for e in self.entities.iter_mut() {
            e.draw(renderer);
        }
    }

    fn collide_one(&mut self, i: usize) {
        let (todo, mut rest) = self.entities.split_at_mut(i);
        let mut this = rest[0].as_mut();

        for other in todo {
            /* &** (&Box<Entity>) : &Entity */
            if let Some(energy) = collide(this, &mut **other) {
                this.collide(&mut **other, energy);
                other.collide(this, energy);
            }
        }
    }

    fn collide_all(&mut self) {
        let len = self.entities.len();
        for i in 0..len {
            self.collide_one(i);
        }
    }

    pub fn think(&mut self, dt: f32, input: &Input, hud: &mut Hud) {
        for (i, e) in self.entities.iter_mut().enumerate() {
            let state = e.think(dt, input, hud, &mut self.born);
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

        self.collide_all();
    }
}
