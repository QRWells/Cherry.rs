use rand::prelude::*;

use crate::core::ray::Ray;
use crate::util::math::{f_schlick, saturate, RandomInit, Vec3};

type Color = Vec3;

struct BsdfInfo {
    wi: Vec3,
    n: Vec3,
    wo: Vec3,
}

pub trait Material: Send + Sync {
    fn eval(&self, bsdfInfo: &BsdfInfo) -> Color;
    fn sample(&self, bsdfInfo: &mut BsdfInfo) -> ();
    fn pdf(&self, bsdfInfo: &BsdfInfo) -> f32;
}

pub struct Mirror {}
pub struct Dielectric {}
pub struct Microfacet {}
