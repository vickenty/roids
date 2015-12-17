use std::f32::consts::PI;

use cgmath::{ rad, Vector, Matrix };

#[allow(non_camel_case_types)]
pub type v32 = ::cgmath::Vector2<f32>;
#[allow(non_camel_case_types)]
pub type m32 = ::cgmath::Matrix2<f32>;

pub struct Body {
    pub p: v32,
    pub a: f32,
    pub r: f32,
    pub m: f32,
    pub dp: v32,
    pub da: f32,
}

impl Body {
    pub fn think(&mut self, dt: f32) {
        self.p = self.p + self.dp * dt;
        self.a = self.a + self.da * dt;
    }

    pub fn apply_force_abs(&mut self, f: v32) {
        self.dp = self.dp + f / self.m;
    }

    pub fn apply_force_world(&mut self, f: f32, a: f32) {
        let t = a * PI;
        self.apply_force_abs(v32::new(t.cos() * f, t.sin() * f));
    }

    pub fn apply_force_local(&mut self, f: f32, a: f32) {
        let a = self.a + a;
        self.apply_force_world(f, a);
    }

    pub fn apply_torque(&mut self, t: f32) {
        self.da += t / self.m;
    }

    pub fn to_world(&mut self, p: v32) -> v32 {
        let m = m32::from_angle(rad(self.a * PI));
        self.p + m.mul_v(p)
    }
}

impl Default for Body {
    fn default() -> Body {
        Body {
            p: v32::new(0.0, 0.0),
            a: 0.0,
            r: 0.0,
            m: 1.0,
            dp: v32::new(0.0, 0.0),
            da: 0.0,
        }
    }
}

#[test]
fn test_body() {
    let mut b = Body { m: 2.0, .. Body::default() };

    b.apply_force_local(1.0, 0.0);
    b.apply_torque(1.0);
    assert_eq!(b.dp, v32::new(0.5, 0.0));
    assert_eq!(b.da, 0.5);

    b.think(2.0);

    assert_eq!(b.dp, v32::new(0.5, 0.0));
    assert_eq!(b.da, 0.5);
    assert_eq!(b.p, v32::new(1.0, 0.0));
    assert_eq!(b.a, 1.0);
}
