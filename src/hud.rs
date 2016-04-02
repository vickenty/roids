use cgmath;
use cgmath::{ Vector };

use render;

pub type V32 = cgmath::Vector2<f32>;

pub struct Bar {
    val: f32,
    pos: V32,
    dim: V32,
    dir: V32,
    color: [f32; 4],
}

impl Bar {
    pub fn new(pos: V32, dim: V32, dir: V32, color: [f32; 4]) -> Bar {
        Bar {
            val: 1.0,
            pos: pos,
            dim: dim,
            dir: dir,
            color: color,
        }
    }

    pub fn set(&mut self, val: f32) {
        self.val = val;
    }

    pub fn draw(&self, renderer: &mut render::Renderer) {
        use render::Vertex;

        let i = self.pos;
        let j = self.pos + self.dim - self.dim * self.dir * (1.0 - self.val);

        let mut shape = renderer.create_shape(
            render::ShapeType::UI,
            self.color.clone(),
            &[
                Vertex::new([i.x, i.y]),
                Vertex::new([i.x, j.y]),
                Vertex::new([j.x, i.y]),
                Vertex::new([j.x, j.y])
            ],
        );

        renderer.draw_shape(&mut shape);
    }
}

pub struct Hud {
    energy: Bar,
    health: Bar,
}

impl Hud {
    pub fn new() -> Hud {
        Hud {
            energy: Bar::new(
               V32::new(-280.0, 240.0),
               V32::new(560.0, 10.0),
               V32::new(1.0, 0.0),
               [ 0.9, 0.6, 0.3, 1.0 ]),
            health: Bar::new(
               V32::new(-280.0, 260.0),
               V32::new(560.0, 20.0),
               V32::new(1.0, 0.0),
               [ 0.3, 0.6, 0.9, 1.0 ]),
        }
    }

    pub fn update(&mut self, energy: f32, health: f32) {
        self.energy.set(energy);
        self.health.set(health);
    }

    pub fn draw(&mut self, renderer: &mut render::Renderer) {
        self.energy.draw(renderer);
        self.health.draw(renderer);
    }
}
