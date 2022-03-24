use rand::{distributions::Standard, prelude::Distribution, Rng};
use ultraviolet::Vec2;

pub fn get_random<T>() -> T
where
    Standard: Distribution<T>,
{
    rand::thread_rng().gen::<T>()
}

pub fn get_random_within_circle() -> Vec2 {
    let rad = get_random::<f32>();
    let r = get_random::<f32>();
    Vec2 {
        x: (r * rad.cos()),
        y: (r * rad.sin()),
    }
}
