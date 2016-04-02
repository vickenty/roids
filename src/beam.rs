use physics::Body;
use input::Input;
use hud::Hud;
use entity::{ State, Entity };
use render;

pub struct Beam {
    body: Body,
    state: State,
}

impl Beam {
    pub fn new(body: Body) -> Beam {
        Beam {
            body: body,
            state: State::Alive,
        }
    }
}

impl Entity for Beam {
    fn draw(&mut self, renderer: &mut render::Renderer) {
        let r = self.body.r;
        let mut shape = renderer.create_shape_simple(
            [ 0.3, 0.9, 0.6, 1.0 ],
            &[ [0.0, r], [1.0, r] ]);
        shape.set_transform(self.body.p.x, self.body.p.y, self.body.a);
        renderer.draw_shape(&mut shape);
    }

    fn think(&mut self, dt: f32, _input: &Input, _hud: &mut Hud, _spawn: &mut Vec<Box<Entity>>) -> State {
        self.body.think(dt);
        self.state
    }

    fn collide(&mut self, _other: &mut Entity, _energy: f32) {
        self.state = State::Dead;
    }

    fn take_damage(&mut self, _damage: f32) {
    }

    fn body(&mut self) -> Option<&mut Body> {
        Some(&mut self.body)
    }
}
