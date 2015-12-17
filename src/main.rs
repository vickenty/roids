extern crate cgmath;
#[macro_use]
extern crate gfx;
extern crate glutin;
extern crate gfx_window_glutin;
extern crate time;

mod input;
mod physics;
mod entity;
mod ship;
mod render;

use std::rc::Rc;
use render::Renderer;
use glutin::Event;
use entity::Entity;

fn main() {
    let window = glutin::WindowBuilder::new()
        .with_title("Roids".to_owned())
        .with_gl(glutin::GL_CORE)
        .with_dimensions(600, 600)
        .with_vsync()
        .build()
        .unwrap();

    let mut renderer = Renderer::new(gfx_window_glutin::init(window));
    let mut input = input::Input::new();
    let mut engine = entity::Engine::new();

    let ship_meta = Rc::new(ship::ShipMeta::default());
    let ship = ship::Ship::new(Default::default(), ship_meta.clone(), &mut renderer);
    engine.add(Box::new(ship));

    let mut t0 = time::precise_time_s();

    'main: loop {
        renderer.draw();

        let t1 = time::precise_time_s();
        let dt = t1 - t0;

        engine.think(dt as f32, &input);

        for ev in renderer.get_window().poll_events() {
            if let Event::Closed = ev {
                break 'main;
            }
            input.handle_event(&ev);
        }

        t0 = t1;
    }
}
