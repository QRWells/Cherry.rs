use rand::prelude::*;

use crate::core::ray::Ray;
use crate::util::math::{f_schlick, saturate, RandomInit, Vec3};

type Color = Vec3;

pub trait Material: Send + Sync {
    fn scatter(&self, ray: &Ray, norm: &Vec3) -> Option<(Color, Vec3)>;
}

pub struct Diffuse {
    albedo: Color,
    roughness: f32,
}

impl Diffuse {
    pub fn new(albedo: Color, roughness: f32) -> Self {
        Diffuse { albedo, roughness }
    }
}

impl Material for Diffuse {
    fn scatter(&self, ray: &Ray, norm: &Vec3) -> Option<(Color, Vec3)> {
        let norm = norm.clone();
        let cos = saturate(norm.normalized().dot(ray.dir().clone().normalized() * -1.0));
        // println!("{}", cos);
        let fresnel = f_schlick(cos, 0.04);
        let bounce = if thread_rng().gen::<f32>() > fresnel {
            norm + Vec3::rand(&mut thread_rng())
        } else {
            ray.dir().reflected(norm.clone()) + (Vec3::rand(&mut thread_rng()) * self.roughness)
        };
        Some((self.albedo, bounce))
    }
}
