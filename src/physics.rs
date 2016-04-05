use std::f32::consts::PI;

use cgmath::{ vec2, rad, Vector, EuclideanVector };

pub type V32 = ::cgmath::Vector2<f32>;
pub type M32 = ::cgmath::Matrix2<f32>;

pub struct Body {
    pub p: V32,
    pub a: f32,
    pub r: f32,
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
        wrap(&mut self.p.x, -300.0, 300.0);
        wrap(&mut self.p.y, -300.0, 300.0);
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
            m: 1.0,
            dp: vec2(0.0, 0.0),
            da: 0.0,
        }
    }
}

const REST_FACTOR: f32 = 0.8;
const UNIT_OF_ENERGY: f32 = 1e8;

fn energy(a_dp: &V32, am: f32, b_dp: &V32, bm: f32) -> f32 {
    let c_dp = (a_dp * am + b_dp * bm) / (am + bm);
    (a_dp - c_dp).length2() * am + (b_dp - c_dp).length2() * bm
}

pub fn collide(a: &mut Body, b: &mut Body) -> Option<f32> {
    let dp = a.p - b.p;
    let dist = dp.length() - a.r - b.r;

    if dist < 0.0 {
        let am = a.r.powi(3);
        let bm = b.r.powi(3);

        let energy_before = energy(&a.dp, am, &b.dp, bm);

        let dv = a.dp - b.dp;
        let change = dp * dv.dot(dp) / dp.length2() * 2.0 / (am + bm) * REST_FACTOR;

        a.dp = a.dp - change * bm;
        b.dp = b.dp + change * am;

        let energy_after = energy(&a.dp, am, &b.dp, bm);

        let correction = dp.normalize() * dist / (am + bm);
        a.p = a.p - correction * bm;
        b.p = b.p + correction * am;

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
