use std::f32::consts::PI;

use cgmath::{ vec2, rad, Vector, EuclideanVector };

pub type V32 = ::cgmath::Vector2<f32>;
pub type M32 = ::cgmath::Matrix2<f32>;

pub struct Body {
    pub p: V32,
    pub a: f32,
    pub r: f32,
    pub ρ: f32,
    pub m: f32,
    pub dp: V32,
    pub da: f32,
}

fn wrap(v: &mut f32, lo: f32, hi: f32) {
    let d = hi - lo;
    if *v < lo {
        *v += d;
    }
    if *v > hi {
        *v -= d;
    }
}

impl Body {
    pub fn init(init: Body) -> Body {
        Body {
            m: init.ρ * init.r.powi(3),
            ..init
        }
    }

    pub fn think(&mut self, dt: f32) {
        self.p = self.p + self.dp * dt;
        self.a = self.a + self.da * dt;
        if self.a > 2.0 {
            self.a -= 2.0;
        }
        if self.a < 0.0 {
            self.a += 2.0;
        }

        /* FIXME: hardcoded world size */
        wrap(&mut self.p.x, -350.0, 350.0);
        wrap(&mut self.p.y, -350.0, 350.0);
    }

    pub fn apply_force_abs(&mut self, f: V32) {
        self.dp = self.dp + f / self.m;
    }

    pub fn apply_force_world(&mut self, f: f32, a: f32) {
        let t = a * PI;
        self.apply_force_abs(vec2(t.cos() * f, t.sin() * f));
    }

    pub fn apply_force_local(&mut self, f: f32, a: f32) {
        let a = self.a + a;
        self.apply_force_world(f, a);
    }

    pub fn apply_torque(&mut self, t: f32) {
        self.da += t / self.m;
    }

    pub fn to_world(&mut self, p: V32) -> V32 {
        let m = M32::from_angle(rad(self.a * PI));
        m * p
    }
}

impl Default for Body {
    fn default() -> Body {
        Body {
            p: vec2(0.0, 0.0),
            a: 0.0,
            r: 0.0,
            ρ: 1.0,
            m: 0.0,
            dp: vec2(0.0, 0.0),
            da: 0.0,
        }
    }
}

const REST_FACTOR: f32 = 0.8;
const UNIT_OF_ENERGY: f32 = 1e8;

fn energy(a: &Body, b: &Body) -> f32 {
    let c_dp = (a.dp * a.m + b.dp * b.m) / (a.m + b.m);
    (a.dp - c_dp).length2() * a.m + (b.dp - c_dp).length2() * b.m
}

pub fn collide(a: &mut Body, b: &mut Body) -> Option<f32> {
    let dp = a.p - b.p;
    let dist = dp.length() - a.r - b.r;

    if dist < 0.0 {
        let energy_before = energy(&a, &b);

        let dv = a.dp - b.dp;
        let change = dp * dv.dot(dp) / dp.length2() * 2.0 / (a.m + b.m) * REST_FACTOR;

        a.dp = a.dp - change * b.m;
        b.dp = b.dp + change * a.m;

        let energy_after = energy(&a, &b);

        let correction = dp.normalize() * dist / (a.m + b.m);
        a.p = a.p - correction * b.m;
        b.p = b.p + correction * a.m;

        Some((energy_before - energy_after) / UNIT_OF_ENERGY)
    } else {
        None
    }
}

#[test]
fn test_body() {
    let mut b = Body { m: 2.0, .. Body::default() };

    b.apply_force_local(1.0, 0.0);
    b.apply_torque(1.0);
    assert_eq!(b.dp, vec2(0.5, 0.0));
    assert_eq!(b.da, 0.5);

    b.think(2.0);

    assert_eq!(b.dp, vec2(0.5, 0.0));
    assert_eq!(b.da, 0.5);
    assert_eq!(b.p, vec2(1.0, 0.0));
    assert_eq!(b.a, 1.0);
}
