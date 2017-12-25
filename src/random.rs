use rand;
use rand::Rng;
use rand::ThreadRng;

static mut RNG: Option<ThreadRng> = None;

pub fn init_rng() {
    unsafe {
        RNG = Some(rand::thread_rng());
    }
}

#[inline]
pub fn get_rng() -> &'static mut ThreadRng {
    unsafe {
        RNG.as_mut().unwrap()
    }
}

#[inline]
pub fn rand_double() -> f64 {
    unsafe {
        RNG.as_mut().unwrap().gen::<f64>()
    }
}
