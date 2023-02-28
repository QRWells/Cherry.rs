use nalgebra::Vector3;
use rand::prelude::*;
use rand::Rng;
use std::f32::consts::PI;
use std::f32::EPSILON;
use std::mem::swap;

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

pub fn solve_quadratic(a: f32, b: f32, c: f32) -> Option<(f32, f32)> {
    let mut x0: f32 = 0.0;
    let mut x1: f32 = 0.0;

    let discriminant = b * b - 4.0 * a * c;
    if discriminant.is_sign_negative() {
        return None;
    }

    if discriminant.abs() < EPSILON {
        x0 = -0.5 * b / a;
        x1 = -0.5 * b / a;
    } else {
        let q = if b.is_sign_positive() {
            -0.5 * (b + discriminant.sqrt())
        } else {
            -0.5 * (b - discriminant.sqrt())
        };
        x0 = q / a;
        x1 = c / q;
    }

    if x0 > x1 {
        swap(&mut x0, &mut x1);
    }

    Some((x0, x1))
}

pub fn max_element(a: Vector3<f32>, b: Vector3<f32>) -> Vector3<f32> {
    Vector3::new(a.x.max(b.x), a.y.max(b.y), a.z.max(b.z))
}

pub fn min_element(a: Vector3<f32>, b: Vector3<f32>) -> Vector3<f32> {
    Vector3::new(a.x.min(b.x), a.y.min(b.y), a.z.min(b.z))
}
