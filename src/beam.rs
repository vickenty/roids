use physics::{ Body, V32 };
use input::Input;
use hud::Hud;
use entity::{ State, Entity };
use render;
use boom::Boom;

pub struct Beam {
    body: Body,
    state: State,
    spawn: Option<V32>,
}

impl Beam {
    pub fn new(body: Body) -> Beam {
        Beam {
            body: body,
            state: State::Alive,
            spawn: None,
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

    fn think(&mut self, dt: f32, _input: &Input, _hud: &mut Hud, spawn: &mut Vec<Box<Entity>>) -> State {
        self.body.think(dt);

        self.body.r -= dt;
        if self.body.r < 0.0 {
            self.state = State::Dead;
        }

        if let Some(p) = self.spawn.take() {
            spawn.push(Box::new(Boom::new(p.x, p.y, self.body.a)));
        }

        self.state
    }

    fn collide(&mut self, other: &mut Entity, _energy: f32) {
        use cgmath::EuclideanVector;
        if let Some(ob) = other.body() {
            self.spawn = Some(ob.p + (self.body.p - ob.p).normalize() * ob.r);
        }
        self.state = State::Dead;
    }

    fn body(&mut self) -> Option<&mut Body> {
        Some(&mut self.body)
    }
}
