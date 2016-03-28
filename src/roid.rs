use rand;
use rand::Rng;
use cgmath::{ Vector2 };

use input::Input;
use entity::{ Entity, State };
use physics::{ Body };
use hud::Hud;
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

    fn explode(&mut self, spawn: &mut Vec<Box<Entity>>) {
        use std::f32::consts::PI;

        if self.size <= 20.0 {
            return;
        }

        let mut rng = rand::thread_rng();
        let pieces = rng.gen_range(3, 6);
        let angle = PI / pieces as f32;
        for p in 0..pieces {
            let a = (p as f32) * 2.0 * angle + rng.gen_range(-angle/2.0, angle/2.0);
            let dp = Vector2::new(a.cos(), a.sin());
            let p = self.body.p + dp * self.size;
            let r = self.size / 2.0;
            let da = rng.gen_range(-0.1, 0.1);
            let body = Body { p: p, dp: dp, da: da, r: r, ..Default::default() };
            let roid = Roid::new(body, r);
            spawn.push(Box::new(roid));
        }
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

    fn think(&mut self, dt: f32, input: &Input, hud: &mut Hud, spawn: &mut Vec<Box<Entity>>) -> State {
        self.body.think(dt);

        if self.health <= 0.0 {
            self.state = State::Dead;
            self.explode(spawn);
        }

        self.state
    }

    fn collide(&mut self, other: &mut Entity, energy: f32) {
        self.take_damage(energy / 1e7);
    }

    fn take_damage(&mut self, damage: f32) {
        self.health -= damage;
    }

    fn body(&mut self) -> Option<&mut Body> {
        Some(&mut self.body)
    }
}
