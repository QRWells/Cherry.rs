use nalgebra::Vector3;
use rand::prelude::*;
use rand::Rng;
use std::f32::consts::PI;

#[inline]
pub fn deg_to_rad(deg: f32) -> f32 {
    deg * PI / 180.0
}

pub fn f_schlick(cos: f32, f0: f32) -> f32 {
    f0 + (1f32 - f0) * (1f32 - cos).powf(5f32)
}

pub trait RandomInit {
    fn rand(rng: &mut ThreadRng) -> Self;
}

impl RandomInit for Vector3<f32> {
    fn rand(rng: &mut ThreadRng) -> Self {
        let theta: f32 = rng.gen_range(0.0..2.0 * PI);
        let phi: f32 = rng.gen_range(-1.0..1.0);
        let ophisq = (1.0 - phi * phi).sqrt();
        Vector3::<f32>::new(ophisq * theta.cos(), ophisq * theta.sin(), phi)
    }
}

pub fn saturate(v: f32) -> f32 {
    v.min(1.0).max(0.0)
}
