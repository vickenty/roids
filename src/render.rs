use glutin;
use gfx;
use gfx_window_glutin;
use cgmath;

use gfx::traits::Device;
use gfx::traits::FactoryExt;
use cgmath::{ SquareMatrix, Matrix3, vec3, rad };

use std::f32::consts::PI;

pub mod backend {
    use gfx_device_gl;

    pub type Device = gfx_device_gl::Device;
    pub type Resources = gfx_device_gl::Resources;
    pub type Factory = gfx_device_gl::Factory;
    pub type CommandBuffer = gfx_device_gl::command::CommandBuffer;
}

gfx_vertex_struct! {
    Vertex {
        pos: [f32; 3] = "vertex_pos",
    }
}

impl Vertex {
    pub fn new(p: [f32; 2]) -> Vertex {
        Vertex {
            pos: [ p[0], p[1], 1.0 ],
        }
    }
}

gfx_pipeline!{
    main_pline {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        color: gfx::Global<[f32; 4]> = "shape_color",
        trans: gfx::Global<[[f32; 3]; 3]> = "shape_trans",
        targ_color: gfx::RenderTarget<gfx::format::Rgba8> = "targ_color",
    }
}

pub fn from_polar(p: &[f32; 2]) -> [f32; 2] {
    [
        (PI * p[0]).cos() * p[1],
        (PI * p[0]).sin() * p[1],
    ]
}

pub enum ShapeType {
    Game,
    UI
}

pub struct Shape {
    ty: ShapeType,
    data: main_pline::Data<backend::Resources>,
    slice: gfx::Slice<backend::Resources>,
    transform: Matrix3<f32>,
}

impl Shape {
    pub fn set_transform(&mut self, x: f32, y: f32, r: f32) {
        self.transform = Matrix3::from_angle_z(rad(r * PI));
        self.transform.z = vec3(x, y, 0.0);
    }
}

pub struct Renderer {
    transform: Matrix3<f32>,

    window: glutin::Window,
    device: backend::Device,
    factory: backend::Factory,

    targ_color: gfx::handle::RenderTargetView<
        backend::Resources,
        gfx::format::Rgba8>,

    targ_depth: gfx::handle::DepthStencilView<
        backend::Resources,
        gfx::format::DepthStencil>,

    encoder: gfx::Encoder<
        backend::Resources,
        backend::CommandBuffer>,

    main_state: gfx::PipelineState<
        backend::Resources,
        main_pline::Meta>,

    ui_state: gfx::PipelineState<
        backend::Resources,
        main_pline::Meta>,
}

impl Renderer {
    pub fn new() -> Self {
        let builder = glutin::WindowBuilder::new()
            .with_title("Roids".to_owned())
            .with_gl(glutin::GL_CORE)
            .with_dimensions(600, 600)
            .with_vsync();

        let (window, device, mut factory, targ_color, targ_depth) =
            gfx_window_glutin::init(builder);

        let shaderset = factory.create_shader_set(
            include_bytes!("main_vert.glsl"),
            include_bytes!("main_frag.glsl"),
        ).unwrap();

        let main_state = factory.create_pipeline_state(
            &shaderset,
            gfx::Primitive::LineStrip,
            gfx::state::Rasterizer::new_fill(gfx::state::CullFace::Nothing),
            main_pline::new(),
        ).unwrap();

        let ui_state = factory.create_pipeline_state(
            &shaderset,
            gfx::Primitive::TriangleStrip,
            gfx::state::Rasterizer::new_fill(gfx::state::CullFace::Nothing),
            main_pline::new(),
        ).unwrap();

        let encoder = factory.create_encoder();

        let transform = {
            let scl = 1.0 / 300.0;
            [
                [ scl, 0.0, 0.0 ],
                [ 0.0, scl, 0.0 ],
                [ 0.0, 0.0, 0.0 ],
            ]
        };

        Renderer {
            transform: transform.into(),
            window: window,
            device: device,
            factory: factory,
            targ_color: targ_color,
            targ_depth: targ_depth,
            encoder: encoder,
            main_state: main_state,
            ui_state: ui_state,
        }
    }

    pub fn get_window(&mut self) -> &mut glutin::Window {
        &mut self.window
    }

    pub fn create_shape(&mut self, ty: ShapeType, vertices: &[Vertex]) -> Shape {
        let (vbuf, slice) = self.factory.create_vertex_buffer(vertices);

        let data = main_pline::Data {
            vbuf: vbuf,
            color: [ 1.0; 4 ],
            trans: Matrix3::identity().into(),
            targ_color: self.targ_color.clone(),
        };

        Shape {
            ty: ty,
            data: data,
            slice: slice,
            transform: Matrix3::identity(),
        }
    }

    pub fn create_shape_simple(&mut self, shape: &[[f32; 2]]) -> Shape {
        let vdata: Vec<_> = shape.iter()
            .map(from_polar)
            .map(Vertex::new)
            .collect();
        self.create_shape(ShapeType::Game, &vdata)
    }

    pub fn draw_shape(&mut self, shape: &mut Shape) {
        shape.data.trans = (self.transform * shape.transform).into();

        let pline_state = match shape.ty {
            ShapeType::Game => &self.main_state,
            ShapeType::UI => &self.ui_state,
        };

        self.encoder.draw(&shape.slice, pline_state, &shape.data);
    }

    pub fn clear(&mut self) {
        self.encoder.reset();
        self.encoder.clear(&self.targ_color, [ 0.01, 0.01, 0.02, 1.0 ]);
        self.encoder.clear_depth(&self.targ_depth, 1.0);
    }

    pub fn finish(&mut self) {
        self.device.submit(self.encoder.as_buffer());
        self.window.swap_buffers().unwrap();
        self.device.cleanup();
    }
}

