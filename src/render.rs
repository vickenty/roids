use gfx::{
    Device,
    Factory,
    OwnedStream,
    ClearData,
    PrimitiveType,
};
use gfx::traits::{
    Stream,
    FactoryExt,
};

use gfx::batch::{ Full };
use gfx::device::handle::{ Program };
use glutin::{ Window };
use gfx_window_glutin::{ Output };

pub use gfx::Resources;

gfx_vertex!(
    Vertex {
        vertex_pos@
        pos: [f32; 2],
    }
);

gfx_parameters!(
    ShaderParam {
        shape_color@
        color: [f32; 4],
    }
);

pub type Batch<R> = ::gfx::batch::Full<ShaderParam<R>>;

pub trait Builder<R> 
    where R: Resources
{
    fn new_batch(&mut self, shape: &[[f32; 2]], color: [f32; 4]) -> Batch<R>;
}

pub struct Renderer<D, F>
    where D: Device,
{
    stream: OwnedStream<D, Output<D::Resources>>,
    device: D,
    factory: F,
    program: Program<D::Resources>,
}

impl<D, F> Renderer<D, F> 
    where D: Device,
          F: Factory<D::Resources>,
{
    pub fn new((stream, device, mut factory): (OwnedStream<D, Output<D::Resources>>, D, F)) -> Self {
        let program = factory.link_program(
            include_bytes!("main_vert.glsl"),
            include_bytes!("main_frag.glsl"),
        ).unwrap();

        Renderer {
            stream: stream,
            device: device,
            factory: factory,
            program: program,
        }
    }

    pub fn get_window(&mut self) -> &mut Window
    {
        &mut self.stream.out.window
    }

    pub fn draw(&mut self) {
        self.stream.clear(ClearData {
            color: [ 0.0, 0.0, 0.0, 1.0 ],
            depth: 1.0,
            stencil: 0,
        });

        self.stream.present(&mut self.device);
    }
}

impl<D, F> Builder<D::Resources> for Renderer<D, F> 
    where D: Device,
          F: Factory<D::Resources>,
{
    fn new_batch(&mut self, shape: &[[f32; 2]], color: [f32; 4]) -> Batch<D::Resources> {
        let vertices: Vec<_> = shape.iter().map(|p| Vertex { pos: *p }).collect();
        let mesh = self.factory.create_mesh(&vertices);
        let params = ShaderParam {
            color: color,
            _r: ::std::marker::PhantomData,
        };
        
        let mut batch = Full::new(mesh, self.program.clone(), params).unwrap();
        batch.slice.prim_type = PrimitiveType::Line;

        batch
    }
}
