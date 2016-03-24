extern crate cgmath;
#[macro_use]
extern crate gfx;
extern crate glutin;
extern crate gfx_window_glutin;
extern crate gfx_device_gl;
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
    let mut renderer = Renderer::new();
    let mut input = input::Input::new();
    let mut engine = entity::Engine::new();

    let ship_meta = Rc::new(ship::ShipMeta::default());
    let ship = ship::Ship::new(
        Default::default(),
        ship_meta.clone(),
        renderer.create_ship_shape());

    engine.add(Box::new(ship));

    let mut t0 = time::precise_time_s();


    'main: loop {
        let t1 = time::precise_time_s();
        let dt = t1 - t0;

        for ev in renderer.get_window().poll_events() {
            if let Event::Closed = ev {
                break 'main;
            }
            input.handle_event(&ev);
        }

        engine.think(dt as f32, &input);

        engine.draw(&mut renderer);

        t0 = t1;
    }
}
