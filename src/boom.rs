use entity::{ State, Entity };
use physics;
use render;
use hud::Hud;
use input::Input;

pub struct Boom {
    x: f32,
    y: f32,
    t: f32,
}

impl Boom {
    pub fn new(x: f32, y: f32) -> Boom {
        Boom {
            x: x,
            y: y,
            t: 0.0,
        }
    }
}

impl Entity for Boom {
    fn draw(&mut self, renderer: &mut render::Renderer) {
        renderer.draw_boom(self.x, self.y, 0.0, 10.0, self.t);
    }

    fn think(&mut self, dt: f32, _: &Input, _: &mut Hud, _: &mut Vec<Box<Entity>>) -> State {
        self.t += dt;

        if self.t < 0.5 {
            State::Alive
        } else {
            State::Dead
        }
    }

    fn collide(&mut self, _: &mut Entity, _: f32) {}
    fn take_damage(&mut self, _: f32) {}
    fn body(&mut self) -> Option<&mut physics::Body> { None }
}
