use rand;
use rand::Rng as R;
pub use std::rc::Rc;
pub use std::fmt;
pub use std::f64::consts::PI;
pub use std::f64;


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

#[macro_export]
macro_rules! perrln {
    ($($arg:tt)*) => ({
        use std::io::{Write, stderr};
        writeln!(&mut stderr(), $($arg)*).ok();
    })
}
