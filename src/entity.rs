use input::Input;

#[derive(Copy, Clone)]
pub enum State {
    Alive,
    Dead,
}

pub type Spawn = Vec<Box<Entity>>;

pub trait Entity {
    fn think(&mut self, dt: f32, &Input) -> (State, Spawn);
    fn collide(&mut self, other: &mut Entity);
    fn take_damage(&mut self, f32);
}
