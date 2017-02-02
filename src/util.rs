use rand;
use rand::Rng as R;
pub use std::rc::Rc;

pub struct Rng {
    rng: rand::XorShiftRng,
}

impl Rng {
    pub fn new() -> Rng {
        Rng { rng: rand::weak_rng() }
    }
    #[inline(always)]
    pub fn rand64(&mut self) -> f64 {
        self.rng.gen::<f64>()
    }
}
