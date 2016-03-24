use rand;
use rand::Rng;

use input::Input;
use entity::{ Entity, State };
use physics::{ Body };
use render;

pub struct Roid
{
    body: Body,
    state: State,
    shape: Option<render::Shape>,

    size: f32,
    health: f32,
}

impl Roid {
    pub fn new(body: Body, size: f32) -> Roid {
        Roid {
            body: body,
            state: State::Alive,
            shape: None,

            size: size,
            health: size,
        }
    }

    pub fn make_shape(&self, renderer: &mut render::Renderer) -> render::Shape {
        let mut rng = rand::thread_rng();

        let size = self.size;
        let n = rng.gen_range(size as u32 / 2, size as u32);

        let mut p = Vec::new();
        for _ in 0..n {
            p.push([ rng.gen_range(0.0, 2.0), rng.gen_range(size / 2.0, size) ]);
        }
        p.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let p0 = p[0].clone();
        p.push(p0);

        renderer.create_shape_simple(&p[..])
    }
}

impl Entity for Roid {
    fn draw(&mut self, renderer: &mut render::Renderer) {
        if self.shape.is_none() {
            self.shape = Some(self.make_shape(renderer));
        }
        if let Some(shape) = self.shape.as_mut() {
            shape.set_transform(self.body.p.x, self.body.p.y, self.body.a);
            renderer.draw_shape(shape);
        }
    }

    fn think(&mut self, dt: f32, input: &Input, spawn: &mut Vec<Box<Entity>>) -> State {
        self.body.think(dt);

        self.state
    }

    fn collide(&mut self, other: &mut Entity) {
    }

    fn take_damage(&mut self, damage: f32) {
    }
}
