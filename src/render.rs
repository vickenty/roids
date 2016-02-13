use glutin;
use gfx;
use gfx_window_glutin;
use gfx_device_gl;

use gfx::{
    format,
    state,
};

gfx_vertex_struct! {
    Vertex {
        pos: [f32; 2] = "vertex_pos",
    }
}

gfx_pipeline!{
    Pipeline {
        color: gfx::Global<[f32; 4]> = "shape_color",
        matrix: gfx::Global<[[f32; 2]; 2]> = "view_matrix",
    }
}

pub struct Renderer {
    window: glutin::Window,
    device: gfx_device_gl::Device,
    factory: gfx_device_gl::Factory,

    targ_color: gfx::handle::RenderTargetView<
        gfx_device_gl::Resources,
        gfx::format::Rgba8>,

    targ_depth: gfx::handle::DepthStencilView<
        gfx_device_gl::Resources,
        gfx::format::DepthStencil>,

    encoder: gfx::Encoder<
        gfx_device_gl::Resources,
        gfx_device_gl::command::CommandBuffer>,

    pipeline: gfx::PipelineState<
        gfx_device_gl::Resources,
        Pipeline::Meta>,
}

impl Renderer {
    pub fn new() -> Self {
        use gfx::traits::FactoryExt;

        let builder = glutin::WindowBuilder::new()
            .with_title("Roids".to_owned())
            .with_gl(glutin::GL_CORE)
            .with_dimensions(600, 600)
            .with_vsync();

        let (window, mut device, mut factory, targ_color, targ_depth) =
            gfx_window_glutin::init::<format::Rgba8>(builder);

        let encoder = factory.create_encoder();

        let pipeline = factory.create_pipeline_simple(
            include_bytes!("main_vert.glsl"),
            include_bytes!("main_frag.glsl"),
            state::CullFace::Nothing,
            Pipeline::new(),
        ).unwrap();

        Renderer {
            window: window,
            device: device,
            factory: factory,
            targ_color: targ_color,
            targ_depth: targ_depth,
            encoder: encoder,
            pipeline: pipeline,
        }
    }

    pub fn get_window(&mut self) -> &mut glutin::Window {
        &mut self.window
    }
}
