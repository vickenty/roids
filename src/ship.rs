use std::rc::Rc;

use input::{ Key, Input };
use physics::Body;
use entity::{ Entity, State };
use render::{ Batch, Resources, Builder };

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

            linear_thrust: 6.0,
            linear_power: 3.0,

            angular_thrust: 0.06,
            angular_power: 1.5,
            angular_limit: 0.04,
            angular_damage: 0.01,

            beam_spread: 0.2,
        }
    }
}

pub struct Ship<R> 
    where R: Resources
{
    pub body: Body,
    state: State,

    pub score: u32,
    pub power: u32,
    pub health: f32,
    pub energy: f32,

    meta: Rc<ShipMeta>,
    batch: Batch<R>,
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

impl<R> Ship<R> 
    where R: Resources
{
    pub fn new<B>(body: Body, meta: Rc<ShipMeta>, builder: &mut B) -> Ship<R> 
        where B: Builder<R>
    {
        Ship {
            body: body,
            state: State::Alive,

            score: meta.init_score,
            power: meta.init_power,
            health: meta.max_health,
            energy: meta.max_energy,

            meta: meta,
            batch: builder.new_batch(SHIP_SHAPE, [ 1.0, 1.0, 1.0, 1.0 ]),
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

    fn fire(&mut self, dt: f32, spawn: &mut Vec<Box<Entity<R>>>) {
    }
}

impl<R> Entity<R> for Ship<R> 
    where R: Resources
{
    fn think(&mut self, dt: f32, input: &Input, spawn: &mut Vec<Box<Entity<R>>>) -> State {
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

        self.state
    }

    fn collide(&mut self, other: &mut Entity<R>) {
    }

    fn take_damage(&mut self, damage: f32) {
        if damage >= self.health {
            self.health = 0.0;
            self.state = State::Dead;
        } else {
            self.health -= damage;
        }
    }

    fn get_batch(&mut self) -> &Batch<R> {
        &self.batch
    }
}
