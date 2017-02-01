use rand;
use std::rc::Rc;
use std::cell::RefCell;
pub use rand::Rng;

pub struct ThreadRng {
    rng: Rc<RefCell<rand::XorShiftRng>>
}

impl ThreadRng {
    pub fn rand64(&mut self) -> f64 {
        self.gen::<f64>()
    }
}

impl rand::Rng for ThreadRng {
    fn next_u32(&mut self) -> u32 {
        self.rng.borrow_mut().next_u32()
    }

    fn next_u64(&mut self) -> u64 {
        self.rng.borrow_mut().next_u64()
    }

    #[inline]
    fn fill_bytes(&mut self, bytes: &mut [u8]) {
        self.rng.borrow_mut().fill_bytes(bytes)
    }
}

pub fn thread_rng() -> ThreadRng
{
    // Must wrap in RefCell since it is mutable.
    thread_local!(static THREAD_RNG_KEY: Rc<RefCell<rand::XorShiftRng>> = {
        Rc::new(RefCell::new(rand::weak_rng()))
    });
    ThreadRng{rng: THREAD_RNG_KEY.with(|t| t.clone())}
}
