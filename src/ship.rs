use std::rc::Rc;

use input::{ Key, Input };
use physics::Body;
use entity::{ Entity, State };
use render;
use hud::Hud;

pub struct ShipMeta {
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

    beam_spread: f32,
}

impl Default for ShipMeta {
    fn default() -> ShipMeta {
        ShipMeta {
            init_score: 0,
            init_power: 0,

            max_health: 15.0,
            max_energy: 50.0,

            linear_thrust: 200.0,
            linear_power: 3.0,

            angular_thrust: 4.0,
            angular_power: 1.5,
            angular_limit: 0.04,
            angular_damage: 0.01,

            beam_spread: 0.2,
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

    meta: Rc<ShipMeta>,
    shape: render::Shape,
}

impl Ship
{
    pub fn new(body: Body, meta: Rc<ShipMeta>, shape: render::Shape) -> Ship
    {
        Ship {
            body: body,
            state: State::Alive,

            score: meta.init_score,
            power: meta.init_power,
            health: meta.max_health,
            energy: meta.max_energy,

            meta: meta,
            shape: shape,
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
    }
}

impl Entity for Ship
{
    fn draw(&mut self, renderer: &mut render::Renderer) {
        self.shape.set_transform(self.body.p.x, self.body.p.y, self.body.a);
        renderer.draw_shape(&mut self.shape);
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
        if input.pressed(Key::Fire) {
            self.fire(dt, spawn);
        }

        self.body.think(dt);

        let over = self.body.da.abs() - self.meta.angular_limit;
        if over > 0.0 {
            let damage = over * self.meta.angular_damage;
            self.take_damage(damage);
        }

        hud.update(self.energy / self.meta.max_energy,
                   self.health / self.meta.max_health);

        self.state
    }

    fn collide(&mut self, other: &mut Entity, energy: f32) {
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
