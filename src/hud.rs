use cgmath;
use cgmath::{ Vector };

use render;

pub type V32 = cgmath::Vector2<f32>;

pub struct Bar {
    val: f32,
    pos: V32,
    dim: V32,
    dir: V32,
}

impl Bar {
    pub fn new(pos: V32, dim: V32, dir: V32) -> Bar {
        Bar {
            val: 1.0,
            pos: pos,
            dim: dim,
            dir: dir,
        }
    }

    pub fn set(&mut self, val: f32) {
        self.val = val;
    }

    pub fn draw(&self, renderer: &mut render::Renderer) {
        use render::Vertex;

        let i = self.pos;
        let j = self.pos + self.dim - self.dim * self.dir * (1.0 - self.val);

        let mut shape = renderer.create_shape(render::ShapeType::UI, &[
            Vertex::new([i.x, i.y]),
            Vertex::new([i.x, j.y]),
            Vertex::new([j.x, i.y]),
            Vertex::new([j.x, j.y])
        ]);

        renderer.draw_shape(&mut shape);
    }
}
