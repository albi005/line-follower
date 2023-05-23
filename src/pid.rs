use std::time::Instant;

pub struct Pid {
    prev: Instant,
    sp: f32,
    kp: f32,
    ki: f32,
    kd: f32,
    i: f32,
    prev_e: f32,
}

impl Pid {
    pub fn new(sp: f32, kp: f32, ki: f32, kd: f32) -> Pid {
        Pid {
            prev: Instant::now(),
            sp,
            kp,
            ki,
            kd,
            i: 0.0,
            prev_e: 0.0,
        }
    }

    pub fn update(&mut self, r: f32) -> f32{
        let e = self.sp - r;
        let dt = self.prev.elapsed().as_secs_f32();
        self.prev = Instant::now();
        self.i += e * dt;
        let d = (e - self.prev_e) / dt;
        self.prev_e = e;
        self.kp * e + self.ki * self.i + self.kd * d
    }
}

