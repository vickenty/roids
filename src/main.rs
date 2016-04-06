extern crate cgmath;
#[macro_use]
extern crate gfx;
extern crate glutin;
extern crate gfx_window_glutin;
extern crate gfx_device_gl;
extern crate time;
extern crate rand;

mod input;
mod physics;
mod entity;
mod ship;
mod roid;
mod render;
mod hud;
mod beam;
mod boom;

use std::rc::Rc;
use render::Renderer;
use glutin::Event;
use entity::Entity;
use cgmath::vec2;

fn main() {
    let mut renderer = Renderer::new();
    let mut input = input::Input::new();
    let mut engine = entity::Engine::new();

    let ship_meta = Rc::new(ship::ShipMeta::default());
    let ship = ship::Ship::new(vec2(-100.0, 0.0), ship_meta.clone());

    let rgen = roid::Generator::new();
    let roid = rgen.create_at(vec2(100.0, 0.0));

    engine.add(Box::new(ship));
    engine.add(Box::new(roid));

    let mut hud = hud::Hud::new();

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

        engine.think(dt as f32, &input, &mut hud);

        renderer.clear();

        engine.draw(&mut renderer);

        hud.draw(&mut renderer);

        renderer.finish();

        t0 = t1;
    }
}
