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
        physics::Body { p: physics::v32::new(-100.0, 0.0), r: 20.0, ..Default::default() },
        ship_meta.clone(),
        renderer.create_ship_shape());

    let roid = roid::Roid::new(
        physics::Body { p: physics::v32::new(100.0, 0.0), r: 150.0, ..Default::default() },
        180.0
    );

    engine.add(Box::new(ship));
    engine.add(Box::new(roid));

    let mut bar = hud::Bar::new(
        hud::V32::new(-280.0, 260.0),
        hud::V32::new( 560.0,  20.0),
        hud::V32::new(   1.0,   0.0),
    );
    bar.set(1.0);

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

        renderer.clear();

        engine.draw(&mut renderer);

        bar.draw(&mut renderer);

        renderer.finish();

        t0 = t1;
    }
}
