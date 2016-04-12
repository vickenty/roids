use time;

pub struct Timer {
    last: f64,
    next: f64,
    dt_min: f64,
    dt_max: f64,
}

impl Timer {
    pub fn new(fps_min: f64, fps_max: f64) -> Timer {
        Timer {
            last: 0.0,
            next: 0.0,
            dt_min: 1.0 / fps_max,
            dt_max: 1.0 / fps_min,
        }
    }

    pub fn are_we_yet(&mut self) -> Option<f64> {
        let time = time::precise_time_s();
        if time >= self.next {
            let dt = time - self.last;
            self.next = time + self.dt_min;
            self.last = time;
            Some(if dt > self.dt_max { self.dt_max } else { dt })
        } else {
            None
        }
    }
}
