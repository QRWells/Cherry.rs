use nalgebra::Vector2;
use rand::{distributions::Standard, prelude::Distribution, Rng};

pub fn get_random<T>() -> T
where
    Standard: Distribution<T>,
{
    rand::thread_rng().gen::<T>()
}

pub fn get_random_within_circle() -> Vector2<f32> {
    let rad = get_random::<f32>();
    let r = get_random::<f32>();
    Vector2::new(r * rad.cos(), r * rad.sin())
}
