use glutin;
use gfx;
use gfx_window_glutin;

use gfx::traits::Device;
use gfx::traits::FactoryExt;
use cgmath::{ Decomposed, Rotation3, Basis3, SquareMatrix, Matrix4, vec3, rad };

use std::f32::consts::PI;

pub mod backend {
    use gfx_device_gl;

    pub type Device = gfx_device_gl::Device;
    pub type Resources = gfx_device_gl::Resources;
    pub type Factory = gfx_device_gl::Factory;
    pub type CommandBuffer = gfx_device_gl::CommandBuffer;
}

gfx_vertex_struct! {
    Vertex {
        pos: [f32; 3] = "vertex_pos",
    }
}

impl Vertex {
    pub fn new(p: [f32; 2]) -> Vertex {
        Vertex {
            pos: [ p[0], p[1], 0.0 ],
        }
    }
}

gfx_pipeline!{
    main_pline {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        color: gfx::Global<[f32; 4]> = "shape_color",
        trans: gfx::Global<[[f32; 4]; 4]> = "shape_trans",
        time: gfx::Global<f32> = "effect_time",
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
    transform: Matrix4<f32>,
}

impl Shape {
    pub fn set_transform(&mut self, x: f32, y: f32, r: f32) {
        let d = Decomposed {
            scale: 1.0,
            rot: Basis3::from_angle_z(rad(r * PI)),
            disp: vec3(x, y, 0.0),
        };
        self.transform = Matrix4::from(d).into();
    }
}

pub struct Renderer {
    transform: Matrix4<f32>,

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

    boom_fx: main_pline::Bundle<backend::Resources>,
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

        let main_shaderset = factory.create_shader_set(
            include_bytes!("main_vert.glsl"),
            include_bytes!("main_frag.glsl"),
        ).unwrap();

        let boom_shaders = factory.create_shader_set(
            include_bytes!("boom_vert.glsl"),
            include_bytes!("boom_frag.glsl"),
        ).unwrap();

        let main_state = factory.create_pipeline_state(
            &main_shaderset,
            gfx::Primitive::LineStrip,
            gfx::state::Rasterizer::new_fill(gfx::state::CullFace::Nothing),
            main_pline::new(),
        ).unwrap();

        let ui_state = factory.create_pipeline_state(
            &main_shaderset,
            gfx::Primitive::TriangleStrip,
            gfx::state::Rasterizer::new_fill(gfx::state::CullFace::Nothing),
            main_pline::new(),
        ).unwrap();

        let boom_fx = {
            let state = factory.create_pipeline_state(
                &boom_shaders,
                gfx::Primitive::TriangleStrip,
                gfx::state::Rasterizer::new_fill(gfx::state::CullFace::Nothing),
                main_pline::new(),
            ).unwrap();

            let vertices = [
                Vertex::new([-1.0, -1.0]),
                Vertex::new([-1.0,  1.0]),
                Vertex::new([ 1.0, -1.0]),
                Vertex::new([ 1.0,  1.0]),
            ];

            let (vbuf, slice) = factory.create_vertex_buffer(&vertices);

            let data = main_pline::Data {
                vbuf: vbuf,
                color: [ 1.0, 1.0, 1.0, 1.0 ],
                time: 0.0,
                trans: Matrix4::identity().into(),
                targ_color: targ_color.clone(),
            };

            main_pline::bundle(slice, state, data)
        };

        let command_buffer = factory.create_command_buffer();

        let transform = {
            let scl = 1.0 / 300.0;
            Matrix4::from_scale(scl)
        };

        Renderer {
            transform: transform,
            window: window,
            device: device,
            factory: factory,
            targ_color: targ_color,
            targ_depth: targ_depth,
            encoder: command_buffer.into(),
            main_state: main_state,
            ui_state: ui_state,
            boom_fx: boom_fx,
        }
    }

    pub fn get_window(&mut self) -> &mut glutin::Window {
        &mut self.window
    }

    pub fn create_shape(&mut self, ty: ShapeType, color: [f32; 4], vertices: &[Vertex]) -> Shape {
        let (vbuf, slice) = self.factory.create_vertex_buffer(vertices);

        let data = main_pline::Data {
            vbuf: vbuf,
            color: color,
            time: 0.0,
            trans: Matrix4::identity().into(),
            targ_color: self.targ_color.clone(),
        };

        Shape {
            ty: ty,
            data: data,
            slice: slice,
            transform: Matrix4::identity(),
        }
    }

    pub fn create_shape_simple(&mut self, color: [f32; 4], shape: &[[f32; 2]]) -> Shape {
        let vdata: Vec<_> = shape.iter()
            .map(from_polar)
            .map(Vertex::new)
            .collect();
        self.create_shape(ShapeType::Game, color, &vdata)
    }

    pub fn draw_shape(&mut self, shape: &mut Shape) {
        shape.data.trans = (self.transform * shape.transform).into();

        let pline_state = match shape.ty {
            ShapeType::Game => &self.main_state,
            ShapeType::UI => &self.ui_state,
        };

        self.encoder.draw(&shape.slice, pline_state, &shape.data);
    }

    pub fn draw_boom(&mut self, x: f32, y: f32, a: f32, r: f32, t: f32) {
        let d = Decomposed {
            scale: r,
            rot: Basis3::from_angle_z(rad(a * PI)),
            disp: vec3(x, y, 0.0),
        };
        self.boom_fx.data.trans = (self.transform * Matrix4::from(d)).into();
        self.boom_fx.data.time = t;
        self.boom_fx.encode(&mut self.encoder);
    }

    pub fn clear(&mut self) {
        self.encoder.clear(&self.targ_color, [ 0.01, 0.01, 0.02, 1.0 ]);
        self.encoder.clear_depth(&self.targ_depth, 1.0);
    }

    pub fn finish(&mut self) {
        self.encoder.flush(&mut self.device);
        self.window.swap_buffers().unwrap();
        self.device.cleanup();
    }
}

