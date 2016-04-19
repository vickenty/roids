use std::rc::Rc;

use cgmath::{ Vector2, vec2 };
use input::{ Key, Input };
use physics::Body;
use entity::{ Entity, State };
use render;
use hud::Hud;
use beam::Beam;

pub struct ShipMeta {
    body_radius: f32,

    init_score: u32,
    init_power: u32,

    max_health: f32,
    max_energy: f32,

    linear_thrust: f32,
    linear_power: f32,

    angular_thrust: f32,
    angular_power: f32,
    angular_limit: f32,
    angular_damage: f32,

    fire_delay: f32,

    beam_spread: f32,
    beam_speed: f32,
    beam_radius: f32,
}

impl Default for ShipMeta {
    fn default() -> ShipMeta {
        ShipMeta {
            body_radius: 20.0,

            init_score: 0,
            init_power: 0,

            max_health: 15.0,
            max_energy: 50.0,

            linear_thrust: 1.6e6,
            linear_power: 3.0,

            angular_thrust: 3.2e4,
            angular_power: 1.5,
            angular_limit: 4.0,
            angular_damage: 0.01,

            fire_delay: 0.1,

            beam_spread: 0.2,
            beam_speed: 360.0,
            beam_radius: 5.0,
        }
    }
}

pub struct Ship
{
    pub body: Body,
    state: State,

    pub score: u32,
    pub power: u32,
    pub health: f32,
    pub energy: f32,

    fire_delay: f32,

    meta: Rc<ShipMeta>,
    shape: Option<render::Shape>,
}

impl Ship
{
    pub fn new(p: Vector2<f32>, meta: Rc<ShipMeta>) -> Ship
    {
        Ship {
            body: Body::init(Body {
                p: p,
                r: meta.body_radius,
                ..Default::default()
            }),
            state: State::Alive,

            score: meta.init_score,
            power: meta.init_power,
            health: meta.max_health,
            energy: meta.max_energy,

            fire_delay: 0.0,

            meta: meta,
            shape: None,
        }
    }

    fn consume(&mut self, energy: f32) -> f32 {
        let output = if energy > self.energy {
            let avail = self.energy;
            self.energy = 0.0;
            avail
        } else {
            self.energy -= energy;
            energy
        };

        output / energy
    }

    fn accel(&mut self, dt: f32, dir: f32) {
        let energy = self.meta.linear_power * dt;
        let thrust = self.meta.linear_thrust * self.consume(energy) * dt;
        self.body.apply_force_local(thrust, dir);
    }

    fn turn(&mut self, dt: f32, dir: f32) {
        let energy = self.meta.angular_power * dt;
        let torque = self.meta.angular_thrust * self.consume(energy) * dt;
        self.body.apply_torque(torque * dir);
    }

    fn fire(&mut self, dt: f32, spawn: &mut Vec<Box<Entity>>) {
        let meta = &self.meta;

        if self.fire_delay >= 0.0 {
            return;
        }

        self.fire_delay = meta.fire_delay;

        let fwd = self.body.to_world(vec2(1.0, 0.0));
        let ofs = self.body.r + meta.beam_radius + 2.5;
        let body = Body::init(Body {
            p: self.body.p + fwd * ofs,
            dp: self.body.dp + fwd * meta.beam_speed,
            a: self.body.a,
            r: meta.beam_radius,
            ..Default::default()
        });
        let beam = Beam::new(body);
        spawn.push(Box::new(beam));
    }
}

impl Entity for Ship
{
    fn draw(&mut self, renderer: &mut render::Renderer) {
        if self.shape.is_none() {
            self.shape = Some(renderer.create_shape_simple([1.0; 4], SHIP_SHAPE));
        }

        if let Some(shape) = self.shape.as_mut() {
            shape.set_transform(self.body.p.x, self.body.p.y, self.body.a);
            renderer.draw_shape(shape);
        }
    }

    fn think(&mut self, dt: f32, input: &Input, hud: &mut Hud, spawn: &mut Vec<Box<Entity>>) -> State {
        if input.pressed(Key::Forward) {
            self.accel(dt, 0.0);
        }
        if input.pressed(Key::Reverse) {
            self.accel(dt, 1.0);
        }
        if input.pressed(Key::Left) {
            self.turn(dt, 1.0);
        }
        if input.pressed(Key::Right) {
            self.turn(dt, -1.0);
        }

        self.body.think(dt);

        if input.pressed(Key::Fire) {
            self.fire(dt, spawn);
        }


        self.fire_delay -= dt;

        let over = self.body.da.abs() - self.meta.angular_limit;
        if over > 0.0 {
            let damage = over * self.meta.angular_damage;
            self.take_damage(damage);
        }

        hud.update(self.energy / self.meta.max_energy,
                   self.health / self.meta.max_health);

        self.state
    }

    fn collide(&mut self, _other: &mut Entity, energy: f32) {
        self.take_damage(energy);
    }

    fn take_damage(&mut self, damage: f32) {
        if damage >= self.health {
            self.health = 0.0;
            self.state = State::Dead;
        } else {
            self.health -= damage;
        }
    }

    fn body(&mut self) -> Option<&mut Body> {
        Some(&mut self.body)
    }
}

const SHIP_SHAPE: &'static [[f32; 2]] = &[
    [0.05, 15.0],
    [0.6, 5.0],
    [0.7, 15.0],
    [0.8, 20.0],
    [0.85, 10.0],
    [1.15, 10.0],
    [1.2, 20.0],
    [1.3, 15.0],
    [1.4, 5.0],
    [1.95, 15.0],
    [0.05, 15.0],
];
